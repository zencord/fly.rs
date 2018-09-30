console.log("ha");

const coll = flyData.collection("testing")
coll.put("id", { foo: "bar" }).then(b => {
  console.log("put returned:", b);
  coll.get("id").then(d => {
    console.log("get returned:", d)
    coll.del("id").then(b => {
      console.log("del returned:", b)
      coll.get("id").then(d => {
        console.log("get returned:", d);
      }).catch(console.log)
    }).catch(console.log)
  }).catch(console.log)
}).catch(console.log)

// console.log(new TextDecoder().decode(new Uint8Array([104, 101, 108, 108, 111])))

let now = Date.now();
setTimeout(() => { console.log("in timeout!", Date.now() - now); now = Date.now() }, 100)

let arr = new Uint8Array(32);
crypto.getRandomValues(arr);
console.log("some random values:", arr);

addEventListener("fetch", function (event) {
  const req = event.request;
  // console.log("req url:", event.request.url);
  let url = new URL(req.url)
  if (url.pathname.endsWith("echo"))
    event.respondWith(new Response(req.body, { headers: { foo: "bar" } }))
  else if (url.pathname.endsWith("null"))
    event.respondWith(new Response(null, { headers: {} }))
  else {
    req.headers.delete("host");
    let u = url.searchParams.get("url");
    let toFetch = new Request(req)
    toFetch.url = u;

    if (url.searchParams.get("cache")) {
      return event.respondWith(cache.match(toFetch).then(res => {
        if (res)
          return res

        return fetch(toFetch).then(res => {
          try {
            cache.put(toFetch, res.clone())
            return res
          } catch (e) {
            console.log(e.message, e.stack)
            return new Response(null)
          }
        })
      }))
    }
    event.respondWith(fetch(toFetch))
  }
})