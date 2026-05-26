import { useBreakpoints } from '@vueuse/core'

/**
 * Responsive breakpoints for ZYBlog layout.
 * Mobile: < 768px (single column, bottom nav)
 * Tablet: 768px - 1023px (collapsible sidebar)
 * Desktop: >= 1024px (full sidebar + content)
 */
export function useResponsive() {
  const breakpoints = useBreakpoints({
    mobile: 0,
    tablet: 768,
    desktop: 1024,
  })

  const isMobile = breakpoints.smaller('tablet')
  const isTablet = breakpoints.between('tablet', 'desktop')
  const isDesktop = breakpoints.greaterOrEqual('desktop')
  const isMobileOrTablet = breakpoints.smaller('desktop')

  return {
    isMobile,
    isTablet,
    isDesktop,
    isMobileOrTablet,
  }
}
