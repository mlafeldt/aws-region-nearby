import { serve } from "https://deno.land/std@0.218.2/http/server.ts";
import { type HandlerContext, router } from "https://crux.land/router@0.0.12";
// @deno-types="./build/aws_region_nearby_deno.d.ts"
import { handler } from "./build/aws_region_nearby_deno.js";

serve(
  router({
    "/": (req: Request, context: HandlerContext) => {
      const ip = (context.remoteAddr as Deno.NetAddr).hostname;
      const headers = new Headers(req.headers);
      headers.append("X-Forwarded-For", ip);
      return handler(new Request(req, { headers }));
    },
  }),
);
