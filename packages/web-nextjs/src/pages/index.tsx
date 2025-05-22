import { Header } from "@entertainarr/components/header";
import Link from "next/link";

export default function Home() {
  return (
    <div className="bg-gray-100 min-h-screen">
      <Header />
      <main className="max-w-6xl mx-auto p-6">
        <div className="flex flex-row justify-between items-center mb-4">
          <h1 className="text-2xl font-bold text-gray-800">Next episodes</h1>
        </div>
        <div className="flex flex-row justify-between items-center my-4">
          <h1 className="text-2xl font-bold text-gray-800">
            Followed TV Shows
          </h1>
          <Link
            href="/tvshows/search"
            className="text-sm px-4 py-2 rounded bg-blue-500 text-white"
          >
            Search
          </Link>
        </div>
      </main>
    </div>
  );
}
