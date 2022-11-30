import { useEffect, useState } from "react"

type Dimensions = {
  width?: number
  height?: number
}

export function useScreenSize(): Dimensions {
  const isBrowser = typeof window !== "undefined"
  const [dimensions, setDimensions] = useState(
    isBrowser ? { width: window.innerWidth, height: window.innerHeight } : {},
  )

  useEffect(() => {
    function updateWindowSize() {
      setDimensions({
        width: window.innerWidth,
        height: window.innerHeight,
      })
    }

    window.addEventListener("resize", updateWindowSize)

    return () => {
      window.removeEventListener("resize", updateWindowSize)
    }
  })

  return dimensions
}
