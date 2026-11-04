/// Keeps child processes from inheriting a socket.
///
/// Sockets created through `mio` (and so `tokio`) are inheritable on
/// Windows. A child process started afterwards, such as the watcher,
/// would otherwise hold on to the socket and keep its port taken after
/// this process has ended.
///
/// # Platform-specific
///
/// - **Windows**: Clears `HANDLE_FLAG_INHERIT` on the socket's handle.
/// - **macOS**: No-op. Child processes started through `std::process` do
///   not inherit sockets.
#[cfg(target_os = "windows")]
pub fn disable_socket_inheritance(
  socket: &impl std::os::windows::io::AsRawSocket,
) -> crate::Result<()> {
  use windows::Win32::Foundation::{
    SetHandleInformation, HANDLE, HANDLE_FLAGS, HANDLE_FLAG_INHERIT,
  };

  #[allow(clippy::cast_possible_wrap)]
  let handle = HANDLE(socket.as_raw_socket() as isize);

  // SAFETY: The handle belongs to a socket that is open for the duration
  // of this call, and only its inheritance flag is changed.
  unsafe {
    SetHandleInformation(handle, HANDLE_FLAG_INHERIT.0, HANDLE_FLAGS(0))
  }?;

  Ok(())
}

/// Keeps child processes from inheriting a socket.
///
/// # Platform-specific
///
/// - **Windows**: Clears `HANDLE_FLAG_INHERIT` on the socket's handle.
/// - **macOS**: No-op. Child processes started through `std::process` do
///   not inherit sockets.
#[cfg(not(target_os = "windows"))]
#[allow(clippy::unnecessary_wraps)]
pub fn disable_socket_inheritance<T>(_socket: &T) -> crate::Result<()> {
  Ok(())
}
