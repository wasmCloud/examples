defmodule Wasmcloudchat.Repo do
  use Ecto.Repo,
    otp_app: :wasmcloudchat,
    adapter: Ecto.Adapters.Postgres
end
