import React, { useState, useEffect } from 'react';
import { documents as documentsApi } from '../api/client';

interface Document {
  id: string;
  document_number: string;
  title: string;
  status: string;
  version: number;
  file_name: string;
}

interface Folder {
  id: string;
  name: string;
  path: string;
}

const Documents: React.FC = () => {
  const [folders, setFolders] = useState<Folder[]>([]);
  const [documents, setDocuments] = useState<Document[]>([]);
  const [selectedFolder, setSelectedFolder] = useState<string | null>(null);
  const [showCreateFolder, setShowCreateFolder] = useState(false);
  const [newFolderName, setNewFolderName] = useState('');
  const [showUpload, setShowUpload] = useState(false);
  const [newDoc, setNewDoc] = useState({ title: '', file_name: '' });

  const loadFolders = async () => {
    try {
      const res = await documentsApi.listFolders(selectedFolder);
      setFolders(res.data);
    } catch (error) {
      console.error('Failed to load folders:', error);
    }
  };

  const loadDocuments = async () => {
    try {
      const res = await documentsApi.listDocuments(selectedFolder);
      setDocuments(res.data);
    } catch (error) {
      console.error('Failed to load documents:', error);
    }
  };

  useEffect(() => {
    void loadFolders();
    void loadDocuments();
  }, [selectedFolder]);

  const handleCreateFolder = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await documentsApi.createFolder({
        name: newFolderName,
        parent_id: selectedFolder,
      });
      setNewFolderName('');
      setShowCreateFolder(false);
      loadFolders();
    } catch (error) {
      console.error('Failed to create folder:', error);
    }
  };

  const handleUpload = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await documentsApi.createDocument({
        title: newDoc.title,
        file_name: newDoc.file_name,
        file_path: `/uploads/${newDoc.file_name}`,
        file_size: 0,
        mime_type: 'application/octet-stream',
        checksum: 'abc123',
        folder_id: selectedFolder,
      });
      setNewDoc({ title: '', file_name: '' });
      setShowUpload(false);
      loadDocuments();
    } catch (error) {
      console.error('Failed to create document:', error);
    }
  };

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Document Management</h1>
        <div className="space-x-2">
          <button
            onClick={() => setShowCreateFolder(true)}
            className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
          >
            New Folder
          </button>
          <button
            onClick={() => setShowUpload(true)}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Upload Document
          </button>
        </div>
      </div>

      {showCreateFolder && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg shadow-lg w-96">
            <h2 className="text-lg font-semibold mb-4">Create Folder</h2>
            <form onSubmit={handleCreateFolder}>
              <input
                type="text"
                value={newFolderName}
                onChange={(e) => setNewFolderName(e.target.value)}
                placeholder="Folder name"
                className="w-full border rounded px-3 py-2 mb-4"
                required
              />
              <div className="flex justify-end space-x-2">
                <button
                  type="button"
                  onClick={() => setShowCreateFolder(false)}
                  className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  Create
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      {showUpload && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg shadow-lg w-96">
            <h2 className="text-lg font-semibold mb-4">Upload Document</h2>
            <form onSubmit={handleUpload}>
              <input
                type="text"
                value={newDoc.title}
                onChange={(e) => setNewDoc({ ...newDoc, title: e.target.value })}
                placeholder="Document title"
                className="w-full border rounded px-3 py-2 mb-4"
                required
              />
              <input
                type="text"
                value={newDoc.file_name}
                onChange={(e) => setNewDoc({ ...newDoc, file_name: e.target.value })}
                placeholder="File name"
                className="w-full border rounded px-3 py-2 mb-4"
                required
              />
              <div className="flex justify-end space-x-2">
                <button
                  type="button"
                  onClick={() => setShowUpload(false)}
                  className="px-4 py-2 bg-gray-200 rounded hover:bg-gray-300"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                >
                  Upload
                </button>
              </div>
            </form>
          </div>
        </div>
      )}

      <div className="grid grid-cols-4 gap-6">
        <div className="col-span-1 bg-white rounded-lg shadow p-4">
          <h2 className="font-semibold mb-4">Folders</h2>
          <div className="space-y-2">
            <div
              onClick={() => setSelectedFolder(null)}
              className={`p-2 rounded cursor-pointer ${!selectedFolder ? 'bg-blue-100' : 'hover:bg-gray-100'}`}
            >
              Root
            </div>
            {folders.map((folder) => (
              <div
                key={folder.id}
                onClick={() => setSelectedFolder(folder.id)}
                className={`p-2 rounded cursor-pointer ${selectedFolder === folder.id ? 'bg-blue-100' : 'hover:bg-gray-100'}`}
              >
                {folder.name}
              </div>
            ))}
          </div>
        </div>

        <div className="col-span-3 bg-white rounded-lg shadow">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left">Document #</th>
                <th className="px-4 py-3 text-left">Title</th>
                <th className="px-4 py-3 text-left">Status</th>
                <th className="px-4 py-3 text-left">Version</th>
                <th className="px-4 py-3 text-left">File</th>
              </tr>
            </thead>
            <tbody>
              {documents.map((doc) => (
                <tr key={doc.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3">{doc.document_number}</td>
                  <td className="px-4 py-3">{doc.title}</td>
                  <td className="px-4 py-3">
                    <span className={`px-2 py-1 rounded text-xs ${
                      doc.status === 'Published' ? 'bg-green-100 text-green-800' :
                      doc.status === 'Draft' ? 'bg-gray-100 text-gray-800' :
                      'bg-yellow-100 text-yellow-800'
                    }`}>
                      {doc.status}
                    </span>
                  </td>
                  <td className="px-4 py-3">v{doc.version}</td>
                  <td className="px-4 py-3">{doc.file_name}</td>
                </tr>
              ))}
              {documents.length === 0 && (
                <tr>
                  <td colSpan={5} className="px-4 py-8 text-center text-gray-500">
                    No documents found
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default Documents;
