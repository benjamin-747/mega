'use client'

import { Button, UIText } from '@gitmono/ui'

import { useDeleteOrionImage } from '@/hooks/OrionClient/useDeleteOrionImage'
import type { OrionVmImage } from '@/hooks/OrionClient/useGetOrionImages'

function shortDigest(digest?: string | null) {
  if (!digest) return '—'
  const hex = digest.replace(/^sha256:/, '').replace(/^sha512:/, '')
  return hex.length > 12 ? `${hex.slice(0, 12)}…` : hex
}

function formatBytes(n?: number | null) {
  if (n == null || n <= 0) return '—'
  const gb = n / (1024 * 1024 * 1024)
  if (gb >= 1) return `${gb.toFixed(1)} GiB`
  const mb = n / (1024 * 1024)
  return `${mb.toFixed(0)} MiB`
}

type Props = {
  images: OrionVmImage[]
  isLoading: boolean
  error: Error | null
}

export function OrionImagesTable({ images, isLoading, error }: Props) {
  const { mutate: deleteImage, isPending: isDeleting, variables: deletingId } = useDeleteOrionImage()

  if (error) {
    return (
      <UIText size='text-sm' color='text-destructive'>
        Failed to load images: {error.message}
      </UIText>
    )
  }

  if (isLoading) {
    return (
      <UIText size='text-sm' color='text-muted'>
        Loading images…
      </UIText>
    )
  }

  if (!images.length) {
    return (
      <UIText size='text-sm' color='text-muted'>
        No catalog images yet. Build with RustFS upload + register to populate this list.
      </UIText>
    )
  }

  return (
    <div className='overflow-x-auto rounded-md border border-gray-200 dark:border-gray-700'>
      <table className='min-w-full text-left text-sm'>
        <thead className='bg-gray-50 text-xs uppercase text-gray-500 dark:bg-gray-900 dark:text-gray-400'>
          <tr>
            <th className='px-3 py-2 font-medium'>Built</th>
            <th className='px-3 py-2 font-medium'>Rust</th>
            <th className='px-3 py-2 font-medium'>Python</th>
            <th className='px-3 py-2 font-medium'>Buck2</th>
            <th className='px-3 py-2 font-medium'>Kernel</th>
            <th className='px-3 py-2 font-medium'>Digest</th>
            <th className='px-3 py-2 font-medium'>Size</th>
            <th className='px-3 py-2 font-medium' />
          </tr>
        </thead>
        <tbody>
          {images.map((img) => (
            <tr key={img.id} className='border-t border-gray-100 dark:border-gray-800'>
              <td className='whitespace-nowrap px-3 py-2'>{img.built_at || '—'}</td>
              <td className='px-3 py-2'>{img.rust || '—'}</td>
              <td className='px-3 py-2'>{img.python || '—'}</td>
              <td className='px-3 py-2'>{img.buck2 || '—'}</td>
              <td className='max-w-[10rem] truncate px-3 py-2' title={img.kernel || undefined}>
                {img.kernel || '—'}
              </td>
              <td className='px-3 py-2 font-mono text-xs' title={img.digest}>
                {shortDigest(img.digest)}
              </td>
              <td className='px-3 py-2'>{formatBytes(img.size_bytes)}</td>
              <td className='px-3 py-2 text-right'>
                <Button
                  variant='plain'
                  size='sm'
                  disabled={isDeleting}
                  onClick={() => {
                    if (window.confirm(`Delete image ${shortDigest(img.digest)}?`)) {
                      deleteImage(img.id)
                    }
                  }}
                >
                  {isDeleting && deletingId === img.id ? 'Deleting…' : 'Delete'}
                </Button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}
