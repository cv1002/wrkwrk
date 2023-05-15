wrk = {
  scheme  = "http",
  host    = "localhost",
  port    = 80,
  method  = "GET",
  path    = "",
  headers = {},
  body    = nil,
  timeout = 30000,
  version = "HTTP1.1",
}

function wrk.setup()
  if type(setup) == "function" then
    setup()
  end
end

function wrk.init(args)
  if not wrk.headers["Host"] then
    local host = wrk.host
    local port = wrk.port

    host = host:find(":") and ("[" .. host .. "]") or host
    host = port and (host .. ":" .. port) or host

    wrk.headers["Host"] = host
  end

  if type(init) == "function" then
    init(args)
  end

  local req = wrk.format()
  wrk.request = function()
    return req
  end
end

function wrk.format(scheme, host, port, method, url, headers, body, timeout, version)
  local host    = host or wrk.host
  local scheme  = scheme or wrk.scheme
  local port    = port or wrk.port
  local method  = method or wrk.method
  local url     = url or wrk.path
  local headers = headers or wrk.headers
  local body    = body or wrk.body
  local timeout = timeout or wrk.timeout
  local version = version or wrk.version

  if headers ~= nil then
    if not headers["Host"] then
      headers["Host"] = host
    end
  end

  return {
    scheme  = scheme,
    host    = host,
    port    = port,
    method  = method,
    url     = url,
    headers = headers,
    body    = body,
    timeout = timeout,
    version = version,
  }
end

function request()
  return wrk.format()
end
