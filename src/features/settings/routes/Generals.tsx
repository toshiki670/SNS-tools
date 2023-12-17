// import { useState } from "react";

import { FormControl, TextField, Container } from "@mui/material";

// import { Spinner } from '@/components/Elements'
import { ContentLayout } from "@/components/Layout";
// import { formatDate } from '@/utils/format';
// import { Button } from "@/components/Elements";

// import { invoke } from "@tauri-apps/api/tauri";

export const Generals = (): JSX.Element => {
  // return (
  //   <div className="w-full h-48 flex justify-center items-center">
  //     <Spinner size="lg" />
  //   </div>
  // )

  // const [data, setData] = useState<string>("");

  // useEffect(() => {
  //   const test = async (): Promise<void> => {
  //     try {
  //       const result = await invoke("x_get_api");
  //       console.log(result);
  //       setData(result.origin);
  //     } catch (err) {
  //       setData("err");
  //     }
  //   };
  //   void test();
  // },[]);

  return (
    <>
      <ContentLayout title={"Generals"}>
        <span className="text-xs font-bold">{"OK3?"}</span>
        <div className="mt-6 flex flex-col space-y-16"></div>
        {/* <p>{data}</p> */}

        <Container>
          <FormControl>
            <TextField
              required
              id="outlined-password-input"
              label="Password"
              type="password"
              autoComplete="current-password"
            />
          </FormControl>
        </Container>

        {/* <Button
            onClick={() => {
              const test = async (): Promise<void> => {
                try {
                  const result = await invoke("x_get_api");
                  console.log(result);
                  setData(result.origin);
                } catch (err) {
                  setData("err");
                }
              };
              void test();
            }}
          >
          Primary Button
        </Button> */}
      </ContentLayout>
    </>
  );
};
