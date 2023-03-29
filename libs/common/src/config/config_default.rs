//! This module provides the [`CONFIG_DEFAULT`] constant which represents the default content for the `config.toml` file.


/// This constant represents the default content for the `config.toml` file.
pub const CONFIG_DEFAULT: &str =
r#"# Sets whether or not all EULAs for the Minecraft servers get accepted automatically.
# The following line is copied from the vanilla Minecraft server's EULA.
# ' By changing the setting below to TRUE you are indicating your agreement to our EULA https://aka.ms/MinecraftEULA. '
# In other words:, when this function returns true, you are agreeing to any EULA this application automatically accepts.
agree_to_eula = true

# The size of the buffers created by this application. (If set too low, it can cause many different kinds of information to only be partially transmitted.)
buffsize = 100000000

# The port the application should use.
communicator_port = 25564

# The maximum number of times an operation gets retried.
max_tries = 3

# The port the webserver should run on.
website_port = 8080

# Sets how long the application wait to give other tasks a chance to execute.
[cooldown]
secs = 0
nanos = 100000000

# If no player is playing on any server for that duration, the computer running this application gets shut down.
# If the value is 0, no shutdowns will be performed.
[shutdown_time]
secs = 0
nanos = 0"#;