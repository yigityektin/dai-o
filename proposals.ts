export interface Proposal {
  id: number;
  subject: string;
  text: string;
  category: string;
  link: string;
}

const proposals: Proposal[] = [];

export default proposals;
