import React from 'react'
import { Flex, Layout } from 'antd'
import BreadCrumb from '@/components/CodeView/TreeView/BreadCrumb'
import CodeContent from '@/components/CodeView/BlobView/CodeContent'
import { AppLayout } from '@/components/Layout/AppLayout'
import AuthAppProviders from '@/components/Providers/AuthAppProviders'
import { useGetBlob } from '@/hooks/useGetBlob'
import { useRouter } from 'next/router'
import { CommentSection } from '@/components/CodeView/BlobView/CommentSection'
import CommitHistory, { CommitInfo } from '@/components/CodeView/CommitHistory'

const codeStyle = {
  borderRadius: 8,
  width: 'calc(85% - 8px)',
  background: '#fff',
  border: '1px solid #d1d9e0',
  margin: '0 8px'
}

interface Comment {
  id: string
  content: string
  author: {
    id: string
    name: string
    avatar?: string
  }
  createdAt: Date
  replies?: Comment[]
}

function BlobPage() {
  const { path = [] } = useRouter().query as { path?: string[] }
  const new_path = '/' + path.join('/')
  const fileContent = useGetBlob({ path: new_path }).data?.data?? ""
  const mockComments: Comment[] = [
    {
      id: '1',
      content: '这段代码逻辑很清晰，建议可以添加一些错误处理。',
      author: {
        id: '1',
        name: '张三',
        avatar: ''
      },
      createdAt: new Date('2024-12-01 10:30:00'),
      replies: []
    },
    {
      id: '2',
      content: '同意。',
      author: {
        id: '2',
        name: '李四',
        avatar: ''
      },
      createdAt: new Date('2024-12-01 10:30:00'),
      replies: [
        {
          id: '3',
          content: '好的，收到。',
          author: {
            id: '3',
            name: '王五',
            avatar: ''
          },
          createdAt: new Date('2024-12-01 10:30:00')
        }
      ]
    }
  ]
  const commitInfo: CommitInfo = {
    user: {
      avatar_url: 'https://avatars.githubusercontent.com/u/112836202?v=4&size=40',
      name: 'yetianxing2014'
    },
    message: 'feat: migrate campsite to mega',
    hash: '5fe4235',
    date: '3 months ago'
  }

  const handleAddComment = (__content: string, __lineNumber?: number) => {
    //wait for complete
  }

  const handleReplyComment = (__commentId: string, __content: string) => {
    //wait for complete
  }


  return (
    <div style={{overflow: 'auto'}}>
      <Flex vertical gap='middle'>
        <Layout>
          <BreadCrumb path={path} />
        </Layout>
        <Layout className='m-2'>
          <CommitHistory flag={'details'} info={commitInfo}/>
        </Layout>
        <Flex gap='middle' wrap>
          <Layout style={codeStyle}>
            <CodeContent fileContent={fileContent} path={path} />
          </Layout>
          <Layout>
            {/* @ts-ignore */}
            <CommentSection comments={mockComments} onAddComment={handleAddComment} onReplyComment={handleReplyComment} />
          </Layout>
        </Flex>
      </Flex>
    </div>
  )
}

BlobPage.getProviders = (
  page:
    | string
    | number
    | boolean
    | React.ReactElement
    | Iterable<React.ReactNode>
    | React.ReactPortal
    | Promise<React.AwaitedReactNode>
    | null
    | undefined,
  pageProps: React.JSX.IntrinsicAttributes & { children?: React.ReactNode }
) => {
  return (
    <AuthAppProviders {...pageProps}>
      <AppLayout {...pageProps}>{page}</AppLayout>
    </AuthAppProviders>
  )
}

export default BlobPage
