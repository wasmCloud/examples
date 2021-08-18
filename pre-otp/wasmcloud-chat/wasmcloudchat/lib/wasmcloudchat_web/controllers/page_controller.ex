defmodule WasmcloudchatWeb.PageController do
  use WasmcloudchatWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
