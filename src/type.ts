export type FileType = 'Application' | 'TXT' | 'JPG' | 'PNG' | 'ZIP' | 'Directory' | 'PDF' | 'GIF' | 'JS' | 'DOC' | 'XLSX' | 'PPT' | 'LNK';
export type Application = {
  name: string,
  path: string,
  file_type: FileType,
  icon_path: string,
  url: string,
  size: string
}

export type SearchPaylod = {
  status: 'InProgress' | 'Completed' | 'Error';
  data: Array<Application>;
}
