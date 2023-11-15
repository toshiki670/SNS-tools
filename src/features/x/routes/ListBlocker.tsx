import { Spinner } from '@/components/Elements'
import { ContentLayout } from '@/components/Layout'
// import { formatDate } from '@/utils/format';

export const ListBlocker = (): JSX.Element => {
  if (false) {
    return (
      <div className="w-full h-48 flex justify-center items-center">
        <Spinner size="lg" />
      </div>
    )
  }

  return (
    <>
      <ContentLayout title={'ListBlocker'}>
        <span className="text-xs font-bold">{'OK?'}</span>
        <div className="mt-6 flex flex-col space-y-16">

        </div>
      </ContentLayout>
    </>
  )
}
