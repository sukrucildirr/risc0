import type { PropsWithChildren } from "react";
import { Footer } from "./_components/footer";
import { Header } from "./_components/header";

export default function BenchmarksAndReportsLayout({ children }: PropsWithChildren) {
  return (
    <>
      <Header />

      <main className="grow">{children}</main>

      <Footer />
    </>
  );
}
