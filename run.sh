cargo new auto_gippity
cd auto_gippity
cargo add dotenv reqwest serde serde_json tokio crossterm async-trait webbrowser strum strum_macros ai_functions

# Create folders inside src:
# ai_functions, apis, helpers, models
# Create mod.rs in each of the above
# Include these mod in the main.rs file

# ./helpers
# create command_line.rs in helpers.

# create .env
# Copy the apiKey and orgID from platform.openai.com into .env
# Add the unit test for the api call.


cargo test tests_writing_backend_code -- --nocapture
