import React, { useState } from "react";
import proposals, { Proposal } from "../proposals";
import { Select } from "@chakra-ui/react";

interface ProposalItemProps {
  proposal: Proposal;
}

const ProposalItem: React.FC<ProposalItemProps> = ({ proposal }) => (
  <div className="border border-white p-4 mb-4 rounded-lg">
    <div>
      <strong className="text-white">Subject:</strong> {proposal.subject}
    </div>
    <div>
      <strong className="text-white">Text:</strong> {proposal.text}
    </div>
    <div>
      <strong className="text-white">Category:</strong> {proposal.category}
    </div>
    {proposal.link && (
      <div>
        <strong className="text-white">Link:</strong>{" "}
        <a href={proposal.link} target="_blank" rel="noopener noreferrer">
          {proposal.link}
        </a>
      </div>
    )}
  </div>
);

const DaoInterface: React.FC = () => {
  const [showAddProposal, setShowAddProposal] = useState(false);
  const [subject, setSubject] = useState("");
  const [text, setText] = useState("");
  const [category, setCategory] = useState("General");
  const [link, setLink] = useState("");

  const handleAddProposal = () => {
    if (subject.trim() !== "" && text.trim() !== "") {
      const newProposal: Proposal = {
        subject,
        text,
        id: Date.now(),
        category,
        link,
      };
      proposals.push(newProposal);

      setSubject("");
      setText("");
      setCategory("General");
      setLink("");
      setShowAddProposal(false);
    }
  };

  return (
    <div className="flex flex-col items-center h-screen mb-4">
      <div className="w-2/3 mx-4 bg-black rounded-lg p-8 shadow-md mb-8 flex flex-col items-center">
        <div className="flex justify-between w-full mb-4">
          <h1 className="text-2xl font-bold text-center text-white">DAI - O</h1>
          <button
            className="bg-blue-500 text-white py-2 px-4 rounded-full"
            onClick={() => setShowAddProposal(!showAddProposal)}
          >
            {showAddProposal ? "Cancel" : "Add Proposal"}
          </button>
        </div>

        {showAddProposal && (
          <div>
            <div className="mb-4">
              <label className="block text-white text-sm font-bold mb-2">
                Subject:
              </label>
              <input
                type="text"
                value={subject}
                onChange={(e) => setSubject(e.target.value)}
                className="shadow appearance-none border rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline"
              />
            </div>

            <div className="mb-4">
              <label className="block text-white text-sm font-bold mb-2">
                Text:
              </label>
              <textarea
                value={text}
                onChange={(e) => setText(e.target.value)}
                className="shadow appearance-none border rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline"
              />
            </div>

            <div className="mb-4">
              <label className="block text-white text-sm font-bold mb-2">
                Category:
              </label>
              <Select
                value={category}
                onChange={(e) => setCategory(e.target.value)}
                placeholder="Select category"
                className="text-white"
              >
                <option value="Model">Model</option>
                <option value="DataSet">Data Set</option>
                <option value="Edit">Edit</option>
              </Select>
            </div>

            <div className="mb-4">
              <label className="block text-white text-sm font-bold mb-2">
                Link:
              </label>
              <input
                type="text"
                value={link}
                onChange={(e) => setLink(e.target.value)}
                className="shadow appearance-none border rounded w-full py-2 px-3 text-white leading-tight focus:outline-none focus:shadow-outline"
              />
            </div>

            <button
              className="bg-green-500 text-white py-2 px-4 rounded-full"
              onClick={handleAddProposal}
            >
              Save Proposal
            </button>
          </div>
        )}
      </div>

      <div className="w-2/3 mx-4 bg-black rounded-lg p-8 shadow-md">
        <h2 className="text-xl font-bold mb-4 text-white">Proposals</h2>
        <div>
          {proposals.map((proposal) => (
            <ProposalItem key={proposal.id} proposal={proposal} />
          ))}
        </div>
      </div>
    </div>
  );
};

export default DaoInterface;
