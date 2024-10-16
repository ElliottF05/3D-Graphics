import { createClient } from '@supabase/supabase-js';
const supabase = createClient(process.env.SUPABASE_URL, process.env.SUPABASE_ANON_KEY);
export function test() {
    console.log("hi");
    // console.log(process.env.API_KEY);
}
//# sourceMappingURL=supabase.js.map