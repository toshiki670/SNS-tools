import { Helmet } from 'react-helmet-async'

interface HeadProps {
  title?: string
  description?: string
}

export const Head = ({ title = '', description = '' }: HeadProps = {}) => {
  return (
    <Helmet
      title={title ? `${title} | SNS Tools` : undefined}
      defaultTitle="SNS Tools"
    >
      <meta name="description" content={description} />
    </Helmet>
  )
}
