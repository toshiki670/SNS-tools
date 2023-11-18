import { useState, useEffect } from "react";

import { tauriClient } from "@/utils/client";

// import { Spinner } from '@/components/Elements'
import { ContentLayout } from "@/components/Layout";
// import { formatDate } from '@/utils/format';

export const ListBlocker = (): JSX.Element => {
  // return (
  //   <div className="w-full h-48 flex justify-center items-center">
  //     <Spinner size="lg" />
  //   </div>
  // )

  const [data, setData] = useState<string>("");

  useEffect(() => {
    const test = async (): Promise<void> => {
      try {
        const result = await tauriClient.query(["x.getAppName", "aaa"]);
        setData(result);
      } catch (err) {
        setData("err");
      }
    };
    void test();
  }, []);

  return (
    <>
      <ContentLayout title={"ListBlocker"}>
        <span className="text-xs font-bold">{"OK?"}</span>
        <div className="mt-6 flex flex-col space-y-16"></div>
        <p>{data}</p>
      </ContentLayout>
    </>
  );
};
