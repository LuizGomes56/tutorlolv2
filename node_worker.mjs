import { createInterface } from "readline";
import { existsSync, mkdirSync, writeFileSync } from "fs";
import { dirname } from "path";

const rl = createInterface({
    input: process.stdin,
    crlfDelay: Infinity,
});

async function handle(job) {
    const { id, url, path } = job;

    try {
        if (existsSync(path)) {
            return { id, ok: true, cached: true };
        }

        const res = await fetch(url);
        const text = await res.text();

        if (!res.ok) {
            return {
                id,
                ok: false,
                status: res.status,
                final_url: res.url,
                error: `HTTP ${res.status}`,
            };
        }

        mkdirSync(dirname(path), { recursive: true });
        writeFileSync(path, text, "utf8");

        return {
            id,
            ok: true,
            cached: false,
            status: res.status,
            final_url: res.url,
        };
    } catch (err) {
        return {
            id,
            ok: false,
            error: String(err?.stack || err),
        };
    }
}

for await (const line of rl) {
    if (!line.trim()) continue;

    let job;
    try {
        job = JSON.parse(line);
    } catch (err) {
        process.stdout.write(
            JSON.stringify({ ok: false, error: `Invalid JSON: ${String(err)}` }) + "\n"
        );
        continue;
    }

    const result = await handle(job);
    process.stdout.write(JSON.stringify(result) + "\n");
}
