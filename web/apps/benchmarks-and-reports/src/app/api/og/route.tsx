import { ImageResponse } from "next/og";

export const runtime = "edge";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const fontDataBold = await fetch(new URL("../../../../public/fonts/EuropaGroteskSH-Med.otf", import.meta.url)).then(
    (res) => res.arrayBuffer(),
  );
  const fontDataNormal = await fetch(new URL("../../../../public/fonts/EuropaGroteskSH-Reg.otf", import.meta.url)).then(
    (res) => res.arrayBuffer(),
  );
  // can't use variables in filenames, need to be static
  const image1Data = await fetch(new URL("./og-1.jpg", import.meta.url)).then((res) => res.arrayBuffer());
  const image2Data = await fetch(new URL("./og-2.jpg", import.meta.url)).then((res) => res.arrayBuffer());
  const image3Data = await fetch(new URL("./og-3.jpg", import.meta.url)).then((res) => res.arrayBuffer());

  try {
    const hasTitle = searchParams.has("title");
    const hasDescription = searchParams.has("description");
    const title = hasTitle ? searchParams.get("title")?.slice(0, 100) : "Universal Zero Knowledge";
    const imageVersion = Math.floor(Math.random() * 3 + 1).toString(); // number between 1 and 3
    const description = hasDescription
      ? searchParams.get("description")?.slice(0, 100)
      : "Get to market fast with dramatically lower development costs on the first general purpose zkVM";

    return new ImageResponse(
      <div
        style={{
          backgroundColor: "#fdff9d",
          height: "100%",
          width: "100%",
          display: "flex",
          justifyContent: "center",
          flexDirection: "column",
          paddingLeft: 48,
        }}
      >
        <svg
          // @ts-expect-error `tw` is valid
          tw="absolute top-12 left-12 w-[128px] h-[93px]"
          width="163"
          height="118"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M27.3863 13.7419v-.1271c0-8.48805-4.4345-12.965141-12.8219-12.965141H1.19009c-.609217 0-1.1050869.494311-1.1050869 1.101611V42.3698c0 .6073.4958699 1.1017 1.1050869 1.1017h2.98939c.60922 0 1.10509-.4944 1.10509-1.1017V26.7071h5.65293l10.0874 16.2418c.1984.3248.5526.5226.9351.5226h3.3436c.3967 0 .7651-.2119.9634-.565.1984-.3531.1842-.7768-.0283-1.1157l-9.4641-15.2249c6.9422-.7344 10.6117-5.155 10.6117-12.824ZM5.28457 5.52219h9.27983c4.8738 0 7.6931 1.3276 7.6931 8.21971 0 6.8922-2.8193 8.0362-7.6931 8.0362H5.28457V5.52219ZM71.0371.649659H46.2435c-.6092 0-1.105.494311-1.105 1.101611v2.6693c0 .6073.4958 1.10162 1.105 1.10162h9.8041V38.5989h-9.8041c-.6092 0-1.105.4943-1.105 1.1016v2.6693c0 .6073.4958 1.1017 1.105 1.1017h24.7936c.6092 0 1.1051-.4944 1.1051-1.1017v-2.6693c0-.6073-.4959-1.1016-1.1051-1.1016h-9.8608V5.52219h9.8608c.6092 0 1.1051-.49432 1.1051-1.10162v-2.6693c0-.6073-.4959-1.101611-1.1051-1.101611ZM105.351 19.476l-1.941-.6073c-4.4624-1.4123-8.6844-2.7399-8.6844-7.6689 0-4.20879 2.6494-6.34141 7.8774-6.34141h.312c6.092 0 8.854 2.68343 8.968 8.69991 0 .5932.51 1.0875 1.091 1.0875h2.932c.298 0 .581-.1129.794-.3248.212-.2118.326-.4943.311-.7909-.155-8.68581-5.157-13.4595008-14.096-13.4595008h-.312c-8.0049 0-13.0061 4.2652308-13.077 11.1433008 0 8.4598 7.3673 10.8325 12.241 12.3861l.553.1836c.495.1554.991.3107 1.501.4661 4.647 1.4405 9.025 2.7964 9.025 8.0785 0 4.5618-3.06 6.878-9.096 6.878h-.297c-6.8289 0-9.6483-2.5422-9.7617-8.7564 0-.5932-.51-1.0875-1.0909-1.0875h-2.9327c-.2975 0-.5809.1271-.7934.3248-.2125.2119-.3259.5085-.3117.7909.1559 8.7565 5.4404 13.5866 14.8904 13.5866h.297c8.77 0 14.225-4.5053 14.225-11.7505 0-8.9401-7.608-11.2845-12.638-12.8381h.014ZM148.591 4.87253h.241c6.234 0 8.94 3.00826 9.039 10.08397 0 .5932.496 1.0875 1.091 1.0875h2.933c.297 0 .581-.113.779-.3248.213-.2119.326-.4943.326-.7909C162.83 2.59867 155.208 0 148.832 0h-.241c-6.46 0-14.167 2.65518-14.167 15.3379v13.29c0 12.7251 7.707 15.3944 14.167 15.3944h.241c6.376 0 13.998-2.5987 14.168-14.9848 0-.2966-.113-.5791-.326-.7909-.212-.2119-.482-.3249-.779-.3249h-2.933c-.595 0-1.091.4943-1.091 1.0875-.099 7.104-2.791 10.1405-9.039 10.1405h-.241c-6.418 0-9.039-3.0506-9.039-10.5218V15.2673c0-7.38651 2.621-10.40891 9.039-10.40891v.01414ZM27.6696 112.52H5.92211L28.3355 79.4152c.1275-.1836.1842-.3954.1842-.6214v-3.1354c0-.6073-.4959-1.1016-1.1051-1.1016H2.25267c-.60922 0-1.10509.4943-1.10509 1.1016v2.6834c0 .6073.49587 1.1017 1.10509 1.1017H22.6542L.184166 112.605C.0566563 112.789 0 113 0 113.226v3.065c0 .607.49587 1.102 1.10508 1.102H27.6696c.6092 0 1.1051-.495 1.1051-1.102v-2.683c0-.608-.4959-1.102-1.1051-1.102v.014ZM70.7963 74.5709H46.4844c-.6092 0-1.1051.4943-1.1051 1.1016v40.6185c0 .607.4959 1.102 1.1051 1.102h24.3119c.6092 0 1.105-.495 1.105-1.102v-2.683c0-.608-.4958-1.102-1.105-1.102H50.5222V98.3122h15.4712c.6092 0 1.1051-.4944 1.1051-1.1017v-2.6834c0-.6073-.4959-1.1016-1.1051-1.1016H50.5222V79.4152h20.2741c.6092 0 1.105-.4943 1.105-1.1016v-2.6834c0-.6073-.4958-1.1017-1.105-1.1017v.0424ZM116.827 87.5502c0-8.4881-4.434-12.9652-12.822-12.9652H90.6311c-.6092 0-1.1051.4944-1.1051 1.1017v40.6183c0 .608.4959 1.102 1.1051 1.102h2.9894c.6092 0 1.1051-.494 1.1051-1.102v-15.663h5.6534l10.087 16.242c.198.325.552.523.935.523h3.344c.396 0 .765-.212.963-.565.198-.353.184-.777-.028-1.116l-9.464-15.225c6.942-.7342 10.611-5.1547 10.611-12.9508Zm-5.128 0v.1271c0 6.7227-2.82 8.0362-7.694 8.0362h-9.2794V79.4576h9.2794c4.874 0 7.694 1.3276 7.694 8.0926ZM148.832 73.9636h-.241c-6.46 0-14.167 2.6552-14.167 15.3379v13.3045c0 12.725 7.707 15.394 14.167 15.394h.241c6.461 0 14.168-2.669 14.168-15.394V89.3015c0-12.6827-7.707-15.3379-14.168-15.3379Zm9.039 15.2814v13.361c0 7.372-2.706 10.521-9.039 10.521h-.241c-6.418 0-9.039-3.05-9.039-10.521V89.245c0-7.3865 2.621-10.4089 9.039-10.4089h.241c6.333 0 9.039 3.1072 9.039 10.4089Z"
            fill="#000"
          />
        </svg>

        <div tw="text-[92px] tracking-wide mb-6 mt-24 text-black max-w-[65%]" style={{ fontFamily: "EuropaBold" }}>
          {title}
        </div>
        <div tw="text-[32px] tracking-wider text-neutral-700 max-w-[65%]" style={{ fontFamily: "EuropaNormal" }}>
          {description}
        </div>
        <img
          style={{ position: "absolute", right: -42, top: 0, zIndex: -1 }}
          width="404"
          height="701"
          alt=""
          // @ts-expect-error
          src={imageVersion === 1 ? image1Data : imageVersion === 2 ? image2Data : image3Data}
        />
      </div>,
      {
        width: 1200,
        height: 630,
        fonts: [
          {
            name: "EuropaNormal",
            data: fontDataNormal,
            style: "normal",
          },
          {
            name: "EuropaBold",
            data: fontDataBold,
            style: "normal",
          },
        ],
      },
    );
  } catch (error: any) {
    console.error(`${error.message}`);

    return new Response("Failed to generate the image", {
      status: 500,
    });
  }
}
