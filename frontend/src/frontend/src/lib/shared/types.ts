import type { file_status } from "../../../../declarations/backend/backend.did";

export type FileData = {
  file_id: string;
  file_name: string;
  file_status: file_status;
  shared_with: string[];
};
