use super::*;

#[derive(Default)]
struct RecordingWatcher {
    watched: Vec<(PathBuf, RecursiveMode)>,
    unwatched: Vec<PathBuf>,
}

impl Watcher for RecordingWatcher {
    fn new<F: notify::EventHandler>(_: F, _: notify::Config) -> notify::Result<Self> {
        Ok(Self::default())
    }

    fn watch(&mut self, path: &Path, mode: RecursiveMode) -> notify::Result<()> {
        self.watched.push((path.to_path_buf(), mode));
        Ok(())
    }

    fn unwatch(&mut self, path: &Path) -> notify::Result<()> {
        self.unwatched.push(path.to_path_buf());
        Ok(())
    }

    fn kind() -> notify::WatcherKind {
        notify::WatcherKind::NullWatcher
    }
}

#[test]
fn initial_and_changed_manifests_are_applied_to_both_watchers() {
    let dir = tempfile::tempdir().unwrap();
    let first = dir.path().join("watch-a").join("config.toml");
    let second = dir.path().join("watch-b").join("config.toml");
    let first_dir = first.parent().unwrap().to_path_buf();
    let second_dir = second.parent().unwrap().to_path_buf();
    let manifest_state = Arc::new(StdRwLock::new(WatchManifest::default()));
    let mut native = RecordingWatcher::default();
    let mut poll = RecordingWatcher::default();

    apply_watch_manifest(
        Some(&mut native),
        Some(&mut poll),
        &manifest_state,
        WatchManifest::from_source_files(&[first.clone()]),
    );
    assert_eq!(
        native.watched,
        vec![(first_dir.clone(), RecursiveMode::NonRecursive)]
    );
    assert_eq!(
        poll.watched,
        vec![(first.clone(), RecursiveMode::NonRecursive)]
    );

    apply_watch_manifest(
        Some(&mut native),
        Some(&mut poll),
        &manifest_state,
        WatchManifest::from_source_files(&[second.clone()]),
    );
    assert_eq!(native.unwatched, vec![first_dir]);
    assert_eq!(poll.unwatched, vec![first]);
    assert_eq!(
        native.watched.last(),
        Some(&(second_dir, RecursiveMode::NonRecursive))
    );
    assert_eq!(
        poll.watched.last(),
        Some(&(second, RecursiveMode::NonRecursive))
    );
}

#[cfg(target_os = "freebsd")]
#[tokio::test]
async fn freebsd_config_watcher_polls_for_config_updates() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("telemt.toml");
    std::fs::write(&path, "[general]\nlog_level = 'normal'\n").unwrap();
    let initial = Arc::new(ProxyConfig::load(&path).unwrap());
    let cancel = tokio_util::sync::CancellationToken::new();
    let (mut config_rx, _, task) = spawn_config_watcher(
        path.clone(),
        initial,
        None,
        None,
        cancel.clone(),
        None,
        None,
        None,
        None,
    );
    let task = tokio::spawn(task);

    // Wait until the watcher has had time to register the initial source.
    tokio::time::sleep(Duration::from_millis(300)).await;
    std::fs::write(&path, "[general]\nlog_level = 'debug'\n").unwrap();
    tokio::time::timeout(Duration::from_secs(12), config_rx.changed())
        .await
        .expect("FreeBSD polling did not reload the changed config")
        .unwrap();
    assert_eq!(config_rx.borrow().general.log_level, LogLevel::Debug);

    cancel.cancel();
    tokio::time::timeout(Duration::from_secs(5), task)
        .await
        .unwrap()
        .unwrap();
}
