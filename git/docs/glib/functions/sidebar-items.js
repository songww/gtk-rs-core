initSidebarItems({"fn":[["access",""],["application_name","Gets a human-readable name for the application, as set by [`set_application_name()`][crate::set_application_name()]. This name should be localized if possible, and is intended for display to the user. Contrast with `g_get_prgname()`, which gets a non-localized name. If [`set_application_name()`][crate::set_application_name()] has not been called, returns the result of `g_get_prgname()` (which may be [`None`] if `g_set_prgname()` has also not been called)."],["base64_decode",""],["base64_encode",""],["check_version",""],["codeset","Gets the character set for the current locale."],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["console_charset","Obtains the character set used by the console attached to the process, which is suitable for printing output to the terminal."],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ","Gets the list of environment variables for the current process."],["file_set_contents",""],["file_set_contents_full",""],["filename_display_basename",""],["filename_display_name",""],["format_size","Formats a size (for example the size of a file) into a human readable string. Sizes are rounded to the nearest size prefix (kB, MB, GB) and are displayed rounded to the nearest tenth. E.g. the file size 3292528 bytes will be converted into the string “3.2 MB”. The returned string is UTF-8, and may use a non-breaking space to separate the number and units, to ensure they aren’t separated when line wrapped."],["format_size_full","Formats a size."],["host_name","Return a name for the machine."],["hostname_is_ascii_encoded","Tests if `hostname` contains segments with an ASCII-compatible encoding of an Internationalized Domain Name. If this returns [`true`], you should decode the hostname with [`hostname_to_unicode()`][crate::hostname_to_unicode()] before displaying it to the user."],["hostname_is_ip_address","Tests if `hostname` is the string form of an IPv4 or IPv6 address. (Eg, “192.168.0.1”.)"],["hostname_is_non_ascii","Tests if `hostname` contains Unicode characters. If this returns [`true`], you need to encode the hostname with [`hostname_to_ascii()`][crate::hostname_to_ascii()] before using it in non-IDN-aware contexts."],["hostname_to_ascii","Converts `hostname` to its canonical ASCII form; an ASCII-only string containing no uppercase letters and not ending with a trailing dot."],["hostname_to_unicode","Converts `hostname` to its canonical presentation form; a UTF-8 string in Unicode normalization form C, containing no uppercase letters, no forbidden characters, and no ASCII-encoded segments, and not ending with a trailing dot."],["language_names","Computes a list of applicable locale names, which can be used to e.g. construct locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable and always contains the default locale “C”."],["language_names_with_category","Computes a list of applicable locale names with a locale category name, which can be used to construct the fallback locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable and always contains the default locale “C”."],["listenv","Gets the names of all variables set in the environment."],["locale_variants","Returns a list of derived variants of `locale`, which can be used to e.g. construct locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable. This function handles territory, charset and extra locale modifiers. See `setlocale(3)` for information about locales and their format."],["main_current_source","Returns the currently firing source for this thread."],["main_depth","Returns the depth of the stack of calls to [`MainContext::dispatch()`][crate::MainContext::dispatch()] on any [`MainContext`][crate::MainContext] in the current thread. That is, when called from the toplevel, it gives 0. When called from within a callback from [`MainContext::iteration()`][crate::MainContext::iteration()] (or [`MainLoop::run()`][crate::MainLoop::run()], etc.) it returns 1. When called from within a callback to a recursive call to [`MainContext::iteration()`][crate::MainContext::iteration()], it returns 2. And so forth."],["mkdir_with_parents","Create a directory if it doesn’t already exist. Create intermediate parent directories as needed, too."],["mkdtemp","Creates a temporary directory. See the `mkdtemp()` documentation on most UNIX-like systems."],["mkdtemp_full","Creates a temporary directory. See the `mkdtemp()` documentation on most UNIX-like systems."],["mkstemp_full","Opens a temporary file. See the `mkstemp()` documentation on most UNIX-like systems."],["monotonic_time","Queries the system monotonic time."],["num_processors","Determine the approximate number of threads that the system will schedule simultaneously for this process. This is intended to be used as a parameter to [`ThreadPool::new()`][crate::ThreadPool::new()] for CPU bound tasks and similar cases."],["on_error_query","type similar to GObject."],["on_error_stack_trace",""],["os_info","Get information about the operating system."],["random_double",""],["random_double_range",""],["random_int",""],["random_int_range",""],["random_set_seed",""],["real_time","Queries the system wall-clock time."],["reload_user_special_dirs_cache",""],["set_application_name",""],["shell_parse_argv",""],["shell_quote",""],["shell_unquote",""],["spaced_primes_closest",""],["spawn_async",""],["spawn_check_exit_status",""],["spawn_check_wait_status",""],["spawn_command_line_async",""],["system_config_dirs","Returns an ordered list of base directories in which to access system-wide configuration information."],["system_data_dirs","Returns an ordered list of base directories in which to access system-wide application data."],["unicode_script_from_iso15924",""],["unicode_script_to_iso15924",""],["unlink",""],["user_cache_dir","Returns a base directory in which to store non-essential, cached data specific to particular user."],["user_config_dir","Returns a base directory in which to store user-specific application configuration information such as user preferences and settings."],["user_data_dir","Returns a base directory in which to access application data such as icons that is customized for a particular user."],["user_runtime_dir","Returns a directory that is unique to the current user on the local system."],["user_special_dir","Returns the full path of a special directory using its logical id."],["user_state_dir",""],["usleep",""],["uuid_string_is_valid",""],["uuid_string_random",""]]});