# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :wasmcloudchat,
  ecto_repos: [Wasmcloudchat.Repo]

# Configures the endpoint
config :wasmcloudchat, WasmcloudchatWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "Zt1aw+xy6lhkkPNaEI2DyYviTpj0wkoNwTcYdyr5yuV/AjUKNbmEeDXeF8Ri7NN3",
  render_errors: [view: WasmcloudchatWeb.ErrorView, accepts: ~w(html json), layout: false],
  pubsub_server: Wasmcloudchat.PubSub,
  live_view: [signing_salt: "V/6VN3DC"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
