import Link from "next/link";

export const Header = () => {
  return (
    <header className="bg-gray-900 text-white shadow-md">
      <div className="max-w-7xl mx-auto px-4 py-3 flex items-center justify-between">
        <Link href="/" className="text-xl font-semibold tracking-wide">
          Entertainarr
        </Link>
        <nav className="space-x-4 text-sm">
          <Link href="/" className="hover:text-indigo-400 transition">
            Home
          </Link>
          <a href="#" className="hover:text-indigo-400 transition">
            {"Library"}
          </a>
          <a href="#" className="hover:text-indigo-400 transition">
            {"Settings"}
          </a>
        </nav>
      </div>
    </header>
  );
};
