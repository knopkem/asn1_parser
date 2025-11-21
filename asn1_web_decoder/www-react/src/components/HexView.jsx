import { Box, Typography } from '@mui/material'
import { memo } from 'react'

const HexView = memo(({ hexData, highlightStart, highlightEnd }) => {
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

  return (
    <Box sx={{ 
      fontFamily: '"Courier New", monospace',
      fontSize: '0.65rem',
      lineHeight: 1.6,
      whiteSpace: 'pre'
    }}>
      {rows.map((row) => (
        <Box 
          key={row.offset}
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
                    color: highlighted ? 'white' : 'text.primary',
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
      ))}
    </Box>
  )
})

HexView.displayName = 'HexView'

export default HexView
