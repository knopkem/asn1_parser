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
  const bytes = hexData.match(/.{1,2}/g) || []
  
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
        return '#388e3c' // Green for length
      }
    } else {
      // Long form: 0x80 + number of length bytes
      const numLengthBytes = lengthByte & 0x7f
      // Length bytes are from highlightStart + 1 to highlightStart + 1 + numLengthBytes
      if (byteIndex >= highlightStart + 1 && byteIndex <= highlightStart + 1 + numLengthBytes) {
        return '#388e3c' // Green for length bytes
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
        fontSize: '0.65rem',
        lineHeight: 1.6,
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
                fontSize: 'inherit'
              }}
            >
              {row.offset.toString(16).padStart(6, '0')}:
            </Typography>

            {/* Hex bytes */}
            <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap' }}>
              {row.bytes.map((byte, idx) => {
                const byteIndex = row.offset + idx
                const highlighted = isHighlighted(byteIndex)
                const byteColor = getByteColor(byteIndex, bytes)
                
                return (
                  <Typography
                    key={idx}
                    component="span"
                    sx={{
                      display: 'inline-block',
                      padding: '1px 2px',
                      borderRadius: '2px',
                      transition: 'all 0.2s',
                      bgcolor: highlighted ? 'primary.main' : 'transparent',
                      color: highlighted ? byteColor : 'text.primary',
                      fontWeight: highlighted ? 600 : 400,
                      transform: highlighted ? 'scale(1.05)' : 'scale(1)',
                      boxShadow: highlighted ? '0 2px 4px rgba(102, 126, 234, 0.3)' : 'none',
                      fontSize: 'inherit'
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
