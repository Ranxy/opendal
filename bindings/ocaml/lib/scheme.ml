(** Services that OpenDAL supports
  
*)
type scheme =
  | Azblob  (** [Azblob]: Azure Storage Blob services.*)
  | Azdfs  (** - [Azdfs]: Azure Data Lake Storage Gen2. *)
  | Cacache  (** [Cacache]: cacache backend support. *)
  | Cos  (** [Cos]: Tencent Cloud Object Storage services. *)
  | Dashmap  (** [Dashmap]: dashmap backend support. *)
  | Etcd  (** [Etcd]: Etcd Services *)
  | Fs  (** [Fs]: POSIX alike file system. *)
  | Ftp  (** [Ftp]: FTP backend. *)
  | Gcs  (** [Gcs]: Google Cloud Storage backend. *)
  | Ghac  (** [Ghac]: GitHub Action Cache services. *)
  | Hdfs  (** [Hdfs]: Hadoop Distributed File System. *)
  | Http  (** [Http]: HTTP backend. *)
  | Ipfs  (** [Ipfs]: IPFS HTTP Gateway *)
  | Ipmfs  (** [Ipmfs]: IPFS mutable file system *)
  | Memcached  (** [Memcached]: Memcached service support. *)
  | Memory  (** [Memory]: In memory backend support. *)
  | MiniMoka  (** [MiniMoka]: Mini Moka backend support. *)
  | Moka  (** [Moka]: moka backend support. *)
  | Obs  (** [Obs]: Huawei Cloud OBS services. *)
  | Onedrive  (** [Onedrive]: Microsoft OneDrive services. *)
  | Gdrive  (** [Gdrive]: GoogleDrive services. *)
  | Dropbox  (** [Dropbox]: Dropbox services. *)
  | Oss  (** [Oss]: Aliyun Object Storage Services *)
  | Persy  (** [Persy]: persy backend support. *)
  | Redis  (** [Redis]: Redis services *)
  | Rocksdb  (** [Rocksdb]: RocksDB services *)
  | S3  (** [S3]: AWS S3 alike services. *)
  | Sftp  (** [Sftp]: SFTP services *)
  | Sled  (** [Sled]: Sled services *)
  | Supabase  (** [Supabase]: Supabase storage service *)
  | VercelArtifacts
      (** [VercelArtifacts]: Vercel Artifacts service, as known as Vercel Remote Caching. *)
  | Wasabi  (** [Wasabi]: Wasabi service *)
  | Webdav  (** [Webdav]: WebDAV support. *)
  | Webhdfs  (** [Webhdfs]: WebHDFS RESTful API Services *)
  | Redb  (** [Redb]: Redb Services *)
  | Tikv  (** [Tikv]: Tikv Services *)

let scheme_str = function
  | Azblob -> "azblob"
  | Azdfs -> "azdfs"
  | Cacache -> "cacache"
  | Cos -> "cos"
  | Dashmap -> "dashmap"
  | Dropbox -> "dropbox"
  | Etcd -> "etcd"
  | Fs -> "fs"
  | Gcs -> "gcs"
  | Gdrive -> "gdrive"
  | Ghac -> "ghac"
  | Hdfs -> "hdfs"
  | Http -> "http"
  | Ftp -> "ftp"
  | Ipfs -> "ipfs"
  | Ipmfs -> "ipmfs"
  | Memcached -> "memcached"
  | Memory -> "memory"
  | MiniMoka -> "mini_moka"
  | Moka -> "moka"
  | Onedrive -> "onedrive"
  | Obs -> "obs"
  | Persy -> "persy"
  | Redb -> "redb"
  | Redis -> "redis"
  | Rocksdb -> "rocksdb"
  | S3 -> "s3"
  | Sftp -> "sftp"
  | Sled -> "sled"
  | Supabase -> "supabase"
  | Oss -> "oss"
  | VercelArtifacts -> "vercel_artifacts"
  | Wasabi -> "wasabi"
  | Webdav -> "webdav"
  | Webhdfs -> "webhdfs"
  | Tikv -> "tikv"
