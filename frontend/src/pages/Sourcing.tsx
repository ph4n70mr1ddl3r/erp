import React, { useState, useEffect } from 'react';
import { sourcing } from '../api/client';

interface SourcingEvent {
  id: string;
  event_number: string;
  title: string;
  status: string;
  event_type: string;
}

interface Bid {
  id: string;
  bid_number: string;
  status: string;
  total_amount: number;
}

const Sourcing: React.FC = () => {
  const [activeTab, setActiveTab] = useState('events');
  const [events, setEvents] = useState<SourcingEvent[]>([]);
  const [selectedEvent, setSelectedEvent] = useState<string | null>(null);
  const [bids, setBids] = useState<Bid[]>([]);
  
  const [showCreateEvent, setShowCreateEvent] = useState(false);
  
  const [newEvent, setNewEvent] = useState({
    title: '',
    event_type: 'RFQ',
    start_date: '',
    end_date: '',
    currency: 'USD',
    estimated_value: 0,
  });

  useEffect(() => {
    loadEvents();
  }, []);

  useEffect(() => {
    if (selectedEvent) {
      loadBids(selectedEvent);
    }
  }, [selectedEvent]);

  const loadEvents = async () => {
    try {
      const res = await sourcing.listEvents();
      setEvents(res.data);
    } catch (error) {
      console.error('Failed to load events:', error);
    }
  };

  const loadBids = async (eventId: string) => {
    try {
      const res = await sourcing.listBids(eventId);
      setBids(res.data);
    } catch (error) {
      console.error('Failed to load bids:', error);
    }
  };

  const handleCreateEvent = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await sourcing.createEvent({
        ...newEvent,
        start_date: new Date(newEvent.start_date).toISOString(),
        end_date: new Date(newEvent.end_date).toISOString(),
        estimated_value: newEvent.estimated_value * 100,
      });
      setNewEvent({ title: '', event_type: 'RFQ', start_date: '', end_date: '', currency: 'USD', estimated_value: 0 });
      setShowCreateEvent(false);
      loadEvents();
    } catch (error) {
      console.error('Failed to create event:', error);
    }
  };

  const handlePublish = async (eventId: string) => {
    try {
      await sourcing.publishEvent(eventId);
      loadEvents();
    } catch (error) {
      console.error('Failed to publish event:', error);
    }
  };

  const handleAcceptBid = async (bidId: string) => {
    try {
      await sourcing.acceptBid(bidId);
      if (selectedEvent) loadBids(selectedEvent);
    } catch (error) {
      console.error('Failed to accept bid:', error);
    }
  };

  return (
    <div className="p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Sourcing & Auctions</h1>
        <button
          onClick={() => setShowCreateEvent(true)}
          className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
        >
          New Event
        </button>
      </div>

      <div className="flex space-x-4 mb-6">
        <button
          onClick={() => setActiveTab('events')}
          className={`px-4 py-2 rounded ${activeTab === 'events' ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
        >
          Events
        </button>
        <button
          onClick={() => setActiveTab('bids')}
          className={`px-4 py-2 rounded ${activeTab === 'bids' ? 'bg-blue-600 text-white' : 'bg-gray-200'}`}
        >
          Bids
        </button>
      </div>

      {showCreateEvent && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white p-6 rounded-lg shadow-lg w-96">
            <h2 className="text-lg font-semibold mb-4">Create Sourcing Event</h2>
            <form onSubmit={handleCreateEvent}>
              <input
                type="text"
                value={newEvent.title}
                onChange={(e) => setNewEvent({ ...newEvent, title: e.target.value })}
                placeholder="Title"
                className="w-full border rounded px-3 py-2 mb-4"
                required
              />
              <select
                value={newEvent.event_type}
                onChange={(e) => setNewEvent({ ...newEvent, event_type: e.target.value })}
                className="w-full border rounded px-3 py-2 mb-4"
              >
                <option value="RFQ">RFQ</option>
                <option value="RFP">RFP</option>
                <option value="RFI">RFI</option>
                <option value="Auction">Auction</option>
                <option value="Tender">Tender</option>
              </select>
              <input
                type="datetime-local"
                value={newEvent.start_date}
                onChange={(e) => setNewEvent({ ...newEvent, start_date: e.target.value })}
                className="w-full border rounded px-3 py-2 mb-4"
                required
              />
              <input
                type="datetime-local"
                value={newEvent.end_date}
                onChange={(e) => setNewEvent({ ...newEvent, end_date: e.target.value })}
                className="w-full border rounded px-3 py-2 mb-4"
                required
              />
              <input
                type="number"
                value={newEvent.estimated_value}
                onChange={(e) => setNewEvent({ ...newEvent, estimated_value: parseFloat(e.target.value) })}
                placeholder="Estimated Value"
                className="w-full border rounded px-3 py-2 mb-4"
                required
              />
              <div className="flex justify-end space-x-2">
                <button type="button" onClick={() => setShowCreateEvent(false)} className="px-4 py-2 bg-gray-200 rounded">Cancel</button>
                <button type="submit" className="px-4 py-2 bg-blue-600 text-white rounded">Create</button>
              </div>
            </form>
          </div>
        </div>
      )}

      <div className="bg-white rounded-lg shadow">
        {activeTab === 'events' && (
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-4 py-3 text-left">Event #</th>
                <th className="px-4 py-3 text-left">Title</th>
                <th className="px-4 py-3 text-left">Type</th>
                <th className="px-4 py-3 text-left">Status</th>
                <th className="px-4 py-3 text-left">Actions</th>
              </tr>
            </thead>
            <tbody>
              {events.map((event) => (
                <tr key={event.id} className="border-t hover:bg-gray-50">
                  <td className="px-4 py-3">{event.event_number}</td>
                  <td className="px-4 py-3">{event.title}</td>
                  <td className="px-4 py-3">{event.event_type}</td>
                  <td className="px-4 py-3">
                    <span className={`px-2 py-1 rounded text-xs ${
                      event.status === 'Published' ? 'bg-blue-100 text-blue-800' :
                      event.status === 'Awarded' ? 'bg-green-100 text-green-800' :
                      event.status === 'Bidding' ? 'bg-yellow-100 text-yellow-800' :
                      'bg-gray-100 text-gray-800'
                    }`}>
                      {event.status}
                    </span>
                  </td>
                  <td className="px-4 py-3">
                    {event.status === 'Draft' && (
                      <button
                        onClick={() => handlePublish(event.id)}
                        className="text-blue-600 hover:underline mr-2"
                      >
                        Publish
                      </button>
                    )}
                    <button
                      onClick={() => setSelectedEvent(event.id)}
                      className="text-blue-600 hover:underline"
                    >
                      View Bids
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}

        {activeTab === 'bids' && selectedEvent && (
          <div>
            <div className="p-4 border-b flex justify-between items-center">
              <h3 className="font-semibold">Bids for Event</h3>
            </div>
            <table className="w-full">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-4 py-3 text-left">Bid #</th>
                  <th className="px-4 py-3 text-left">Total Amount</th>
                  <th className="px-4 py-3 text-left">Status</th>
                  <th className="px-4 py-3 text-left">Actions</th>
                </tr>
              </thead>
              <tbody>
                {bids.map((bid) => (
                  <tr key={bid.id} className="border-t hover:bg-gray-50">
                    <td className="px-4 py-3">{bid.bid_number}</td>
                    <td className="px-4 py-3">${(bid.total_amount / 100).toFixed(2)}</td>
                    <td className="px-4 py-3">
                      <span className={`px-2 py-1 rounded text-xs ${
                        bid.status === 'Accepted' ? 'bg-green-100 text-green-800' :
                        bid.status === 'Rejected' ? 'bg-red-100 text-red-800' :
                        'bg-gray-100 text-gray-800'
                      }`}>
                        {bid.status}
                      </span>
                    </td>
                    <td className="px-4 py-3">
                      {bid.status === 'Submitted' && (
                        <button
                          onClick={() => handleAcceptBid(bid.id)}
                          className="text-green-600 hover:underline"
                        >
                          Accept
                        </button>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
};

export default Sourcing;
