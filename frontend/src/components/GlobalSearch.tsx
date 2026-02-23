import { useState, useEffect, useRef } from 'react';
import { Search, X, Package, Users, ShoppingCart, Building2, Factory, UserCog, Folder, Headphones, Monitor } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface SearchResult {
  type: string;
  id: string;
  name: string;
  description?: string;
  path: string;
}

const mockSearch = async (query: string): Promise<SearchResult[]> => {
  if (!query.trim()) return [];
  
  const results: SearchResult[] = [];
  const q = query.toLowerCase();
  
  const entities: { type: string; icon: React.ReactNode; path: string; items: { id: string; name: string; desc?: string }[] }[] = [
    { type: 'Products', icon: <Package className="w-4 h-4" />, path: '/inventory', items: [
      { id: '1', name: 'Laptop Pro 15', desc: 'Electronics' },
      { id: '2', name: 'Office Chair', desc: 'Furniture' },
      { id: '3', name: 'Printer Paper A4', desc: 'Supplies' },
    ]},
    { type: 'Customers', icon: <ShoppingCart className="w-4 h-4" />, path: '/sales', items: [
      { id: '1', name: 'Acme Corporation', desc: 'Enterprise' },
      { id: '2', name: 'TechStart Inc', desc: 'SMB' },
    ]},
    { type: 'Vendors', icon: <Users className="w-4 h-4" />, path: '/purchasing', items: [
      { id: '1', name: 'Global Supplies Ltd', desc: 'Office supplies' },
      { id: '2', name: 'TechParts Co', desc: 'Components' },
    ]},
    { type: 'Projects', icon: <Folder className="w-4 h-4" />, path: '/projects', items: [
      { id: '1', name: 'ERP Implementation', desc: 'Active' },
      { id: '2', name: 'Website Redesign', desc: 'In Progress' },
    ]},
  ];
  
  entities.forEach(entity => {
    entity.items.forEach(item => {
      if (item.name.toLowerCase().includes(q) || (item.desc?.toLowerCase().includes(q))) {
        results.push({
          type: entity.type,
          id: item.id,
          name: item.name,
          description: item.desc,
          path: entity.path,
        });
      }
    });
  });
  
  return results.slice(0, 10);
};

const typeIcons: Record<string, React.ReactNode> = {
  'Products': <Package className="w-4 h-4 text-blue-500" />,
  'Customers': <ShoppingCart className="w-4 h-4 text-green-500" />,
  'Vendors': <Users className="w-4 h-4 text-purple-500" />,
  'Projects': <Folder className="w-4 h-4 text-orange-500" />,
  'Finance': <Building2 className="w-4 h-4 text-emerald-500" />,
  'Manufacturing': <Factory className="w-4 h-4 text-red-500" />,
  'HR': <UserCog className="w-4 h-4 text-pink-500" />,
  'Service': <Headphones className="w-4 h-4 text-cyan-500" />,
  'IT Assets': <Monitor className="w-4 h-4 text-indigo-500" />,
};

export default function GlobalSearch() {
  const [isOpen, setIsOpen] = useState(false);
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const navigate = useNavigate();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsOpen(true);
        setTimeout(() => inputRef.current?.focus(), 0);
      }
      if (e.key === 'Escape') {
        setIsOpen(false);
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, []);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (containerRef.current && !containerRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  useEffect(() => {
    const search = async () => {
      if (!query.trim()) {
        setResults([]);
        return;
      }
      setLoading(true);
      try {
        const res = await mockSearch(query);
        setResults(res);
      } catch (error) {
        console.error('Search failed:', error);
      } finally {
        setLoading(false);
      }
    };
    
    const debounce = setTimeout(search, 200);
    return () => clearTimeout(debounce);
  }, [query]);

  const handleSelect = (result: SearchResult) => {
    navigate(result.path);
    setIsOpen(false);
    setQuery('');
    setResults([]);
  };

  return (
    <div ref={containerRef} className="relative">
      <button
        onClick={() => { setIsOpen(true); setTimeout(() => inputRef.current?.focus(), 0); }}
        className="flex items-center gap-2 px-3 py-2 text-gray-500 bg-gray-100 hover:bg-gray-200 rounded-lg text-sm"
      >
        <Search className="w-4 h-4" />
        <span className="hidden sm:inline">Search...</span>
        <kbd className="hidden md:inline-flex items-center gap-1 px-1.5 py-0.5 text-xs bg-white border rounded">
          ⌘K
        </kbd>
      </button>

      {isOpen && (
        <div className="fixed inset-0 z-50 md:absolute md:inset-auto md:top-full md:right-0 md:mt-2 md:w-96">
          <div className="md:bg-white md:rounded-lg md:shadow-lg md:border">
            <div className="flex items-center gap-2 p-3 border-b">
              <Search className="w-5 h-5 text-gray-400" />
              <input
                ref={inputRef}
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search products, customers, projects..."
                className="flex-1 outline-none text-gray-900 placeholder-gray-400"
                autoFocus
              />
              <button onClick={() => setIsOpen(false)}>
                <X className="w-5 h-5 text-gray-400 hover:text-gray-600" />
              </button>
            </div>

            <div className="max-h-80 overflow-y-auto">
              {loading ? (
                <div className="p-4 text-center text-gray-500">Searching...</div>
              ) : results.length > 0 ? (
                <div className="divide-y">
                  {results.map((result, idx) => (
                    <button
                      key={`${result.type}-${result.id}-${idx}`}
                      onClick={() => handleSelect(result)}
                      className="w-full p-3 flex items-center gap-3 hover:bg-gray-50 text-left"
                    >
                      {typeIcons[result.type] || <Search className="w-4 h-4 text-gray-400" />}
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-medium text-gray-900">{result.name}</p>
                        <p className="text-xs text-gray-500">
                          {result.type} {result.description && `• ${result.description}`}
                        </p>
                      </div>
                    </button>
                  ))}
                </div>
              ) : query ? (
                <div className="p-4 text-center text-gray-500">No results found</div>
              ) : (
                <div className="p-4 text-center text-gray-500">Type to search...</div>
              )}
            </div>

            <div className="p-2 border-t text-center hidden md:block">
              <p className="text-xs text-gray-400">
                Press <kbd className="px-1 py-0.5 bg-gray-100 rounded">ESC</kbd> to close
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
