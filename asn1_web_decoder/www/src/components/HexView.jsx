import { Box, Typography } from '@mui/material'
import { memo, useEffect, useRef } from 'react'

const HexView = memo(({ hexData, highlightStart, highlightEnd }) => {
  const containerRef = useRef(null)
  const highlightedRowRef = useRef(null)

  useEffect(() => {
    if (highlightedRowRef.current && containerRef.current) {
      const container = containerRef.current
      const element = highlightedRowRef.current
      
      const containerRect = container.getBoundingClientRect()
      const elementRect = element.getBoundingClientRect()
      
      // Check if element is outside visible area
      if (elementRect.top < containerRect.top || elementRect.bottom > containerRect.bottom) {
        element.scrollIntoView({ behavior: 'smooth', block: 'center' })
      }
    }
  }, [highlightStart, highlightEnd])

  if (!hexData) {
    return (
      <Box sx={{ 
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        flexGrow: 1,
        textAlign: 'center',
        color: 'text.secondary',
        p: 4
      }}>
        <Typography variant="h6">
          Decode PEM data to view hex
        </Typography>
      </Box>
    )
  }

  // Format hex data into groups of 2 characters (1 byte)
  const bytes = hexData.toUpperCase().match(/.{1,2}/g) || []
  
  // Group into 16 bytes per row
  const rows = []
  for (let i = 0; i < bytes.length; i += 16) {
    rows.push({
      offset: i,
      bytes: bytes.slice(i, i + 16)
    })
  }

  const isHighlighted = (byteIndex) => {
    if (highlightStart === null || highlightEnd === null) return false
    return byteIndex >= highlightStart && byteIndex < highlightEnd
  }

  const getByteColor = (byteIndex, bytes) => {
    if (highlightStart === null || highlightEnd === null) return 'text.primary'
    if (byteIndex < highlightStart || byteIndex >= highlightEnd) return 'text.primary'
    
    // Tag byte is always the first byte
    if (byteIndex === highlightStart) {
      return '#d32f2f' // Red for tag
    }
    
    // Parse length bytes
    // The byte after the tag is the length field
    const lengthByte = parseInt(bytes[highlightStart + 1], 16)
    
    if (lengthByte < 0x80) {
      // Short form: length is in this one byte
      if (byteIndex === highlightStart + 1) {
        return '#2e7d32' // Darker green for length
      }
    } else {
      // Long form: 0x80 + number of length bytes
      const numLengthBytes = lengthByte & 0x7f
      // Length bytes are from highlightStart + 1 to highlightStart + 1 + numLengthBytes
      if (byteIndex >= highlightStart + 1 && byteIndex <= highlightStart + 1 + numLengthBytes) {
        return '#2e7d32' // Darker green for length bytes
      }
    }
    
    // Content bytes are not colored (will show as white on purple background when highlighted)
    return 'text.primary'
  }

  // Find the row that contains the highlighted start byte
  const highlightedRowIndex = highlightStart !== null 
    ? Math.floor(highlightStart / 16) 
    : -1

  return (
    <Box 
      ref={containerRef}
      sx={{ 
        fontFamily: '"Courier New", monospace',
        fontSize: '0.6rem',
        lineHeight: 1.3,
        whiteSpace: 'pre'
      }}
    >
      {rows.map((row, rowIndex) => {
        const rowHasHighlight = row.bytes.some((_, idx) => 
          isHighlighted(row.offset + idx)
        )

        return (
          <Box 
            key={row.offset}
            ref={rowIndex === highlightedRowIndex ? highlightedRowRef : null}
            sx={{ 
              display: 'flex',
              gap: 2,
              '&:hover': {
                bgcolor: 'rgba(0, 0, 0, 0.02)'
              }
            }}
          >
            {/* Offset */}
            <Typography
              component="span"
              sx={{ 
                color: 'text.secondary',
                minWidth: '60px',
                userSelect: 'none',
                fontSize: 'inherit',
                textTransform: 'uppercase'
              }}
            >
              {row.offset.toString(16).padStart(6, '0')}:
            </Typography>

            {/* Hex bytes */}
            <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: '4px' }}>
              {row.bytes.map((byte, idx) => {
                const byteIndex = row.offset + idx
                const highlighted = isHighlighted(byteIndex)
                const byteColor = getByteColor(byteIndex, bytes)
                const prevHighlighted = idx > 0 && isHighlighted(row.offset + idx - 1)
                const nextHighlighted = idx < row.bytes.length - 1 && isHighlighted(row.offset + idx + 1)
                
                return (
                  <Typography
                    key={idx}
                    component="span"
                    sx={{
                      display: 'inline-block',
                      padding: '1px 2px',
                      position: 'relative',
                      minWidth: '14px',
                      textAlign: 'center',
                      fontFamily: '"Courier New", monospace',
                      fontVariantNumeric: 'tabular-nums',
                      borderTopLeftRadius: highlighted && !prevHighlighted ? '3px' : 0,
                      borderBottomLeftRadius: highlighted && !prevHighlighted ? '3px' : 0,
                      borderTopRightRadius: highlighted && !nextHighlighted ? '3px' : 0,
                      borderBottomRightRadius: highlighted && !nextHighlighted ? '3px' : 0,
                      bgcolor: highlighted ? 'rgba(102, 126, 234, 0.5)' : 'transparent',
                      color: highlighted ? byteColor : 'text.primary',
                      fontWeight: highlighted ? 600 : 400,
                      fontSize: 'inherit',
                      '&::after': highlighted && nextHighlighted ? {
                        content: '""',
                        position: 'absolute',
                        right: '-4px',
                        top: 0,
                        bottom: 0,
                        width: '4px',
                        bgcolor: 'rgba(102, 126, 234, 0.5)',
                        zIndex: 0
                      } : {}
                    }}
                  >
                    {byte}
                  </Typography>
                )
              })}
            </Box>
          </Box>
        )
      })}
    </Box>
  )
})

HexView.displayName = 'HexView'

export default HexView
