wrk = {
  scheme  = "http",
  host    = "localhost",
  port    = 80,
  method  = "GET",
  path    = "",
  headers = {},
  body    = nil,
  thread  = nil,
  timeout = 30000,
  version = "HTTP2",
}

function wrk.setup(thread)
  if type(setup) == "function" then
    setup(thread)
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
  if headers ~= nil and not headers["Host"] then
    headers["Host"] = wrk.headers["Host"]
  end
  local host    = host or headers["Host"]

  local scheme  = scheme or wrk.scheme
  local port    = port or wrk.port
  local method  = method or wrk.method
  local url     = url or wrk.path
  local headers = headers or wrk.headers
  local body    = body or wrk.body
  local timeout = timeout or wrk.timeout
  local version = version or wrk.version

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
  return wrk.format(
    "https",
    "www.qq.com",
    443,
    "GET",
    "",
    nil,
    nil,
    nil,
    nil
  )
end

function init()
end
