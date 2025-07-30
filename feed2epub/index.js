// DISCLOSURE: llm generated quickscope
import fs from 'fs';
import { parseStringPromise } from 'xml2js';
import axios from 'axios';
import Epub from 'epub-gen';
import { program } from 'commander';
import createDOMPurify from 'dompurify';
import { JSDOM } from 'jsdom';

const window = new JSDOM('').window;
const DOMPurify = createDOMPurify(window);

async function fetchFeed(url) {
    const res = await axios.get(url);
    return await parseStringPromise(res.data);
}

function extractEntries(feed) {
    if (feed.rss) {
        return feed.rss.channel[0].item;
    } else if (feed.feed && feed.feed.entry) {
        return feed.feed.entry;
    } else {
        throw new Error("Unsupported feed format");
    }
}

function formatContent(content) {
    return DOMPurify.sanitize(content ?? "")
}

async function processFeeds(urls, output) {
    const allEntries = [];

    for (const url of urls) {
        const feed = await fetchFeed(url);
        const entries = extractEntries(feed);

        for (const entry of entries) {
            const title = typeof entry.title?.[0] === 'string' ? entry.title[0] : entry.title?.[0]?.toString('utf8') || 'Untitled';
            const rawContent = entry['content:encoded']?.[0] ||
                entry.content?.[0]?._ ||
                entry.description?.[0] ||
                'No content';
            const content = formatContent(typeof rawContent === 'string' ? rawContent : rawContent.toString('utf8'));
            allEntries.push({ title, data: `<h3>${title}</h3>${content}` });
        }
    }

    const option = {
        title: "Feeds",
        author: process.env.USER || "nobody",
        language: "en-US",
        output,
        content: allEntries
    };

    await new Epub(option).promise;
    console.log(`Saved to ${output}`);
}

// CLI
program
    .requiredOption('-o, --output <path>', 'Output EPUB file path')
    .argument('<feeds...>', 'Feed URLs')
    .action(async (feeds, opts) => {
        try {
            await processFeeds(feeds, opts.output);
        } catch (e) {
            console.error("Failed:", e.message);
            process.exit(1);
        }
    });

program.parse();

