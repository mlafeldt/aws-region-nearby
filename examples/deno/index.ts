import { serve } from "https://deno.land/std@0.153.0/http/server.ts";
// @deno-types="./build/aws_region_nearby_deno.d.ts"
import { handler } from "./build/aws_region_nearby_deno.js";

serve(handler);
