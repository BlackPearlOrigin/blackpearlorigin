// I should really put these on a typings file
// Maybe it will be global, idk i might have to search
// But for the meanwhile this works
// - Brisolo32

export interface Config {
    currentLang: string;
}

interface Links {
    link: string;
}

export interface SearchedGame {
    name: string;
    links: Links[];
    scraper: string;
}

export interface Game {
    id: number;
    name: string;
    exe_path: string;
    description: string;
    image: string;
}

export interface IGDBData {
    cover_url: string;
    id: number;
    name: string;
    summary: string;
}

export interface ScraperResponseEntry {
    links: string[];
    name: string;
    scraper: string;
}
