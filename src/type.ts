export type FileType = 'Application' | 'JPG' | 'PNG' | 'Folder' | 'PDF' | 'GIF' | 'JS';
export type Application = {
  name: string,
  text_name: string,
  r_type: FileType,
  r_install: string,
  r_publisher: string,
  r_icon_buffer: Array<number>,
  r_icon_path: string,
  r_exe_path: string,
  url: string
}

export type SearchPaylod = {
  status: 'InProgress' | 'Completed' | 'Error';
  data: Array<Application>;
}
