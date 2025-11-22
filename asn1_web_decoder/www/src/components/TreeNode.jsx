import { useState } from 'react'
import { Box, IconButton, Typography } from '@mui/material'
import { ExpandMore, ChevronRight } from '@mui/icons-material'

function TreeNode({ node, onNodeHover }) {
  const [isCollapsed, setIsCollapsed] = useState(false)
  const hasChildren = node.children && node.children.length > 0

  const toggleCollapse = (e) => {
    e.stopPropagation()
    setIsCollapsed(!isCollapsed)
  }

  const handleMouseEnter = () => {
    if (onNodeHover) {
      onNodeHover(node)
    }
  }

  const handleMouseLeave = () => {
    if (onNodeHover) {
      onNodeHover(null)
    }
  }

  // Get color for tag class
  const getTagClassColor = (tagClass) => {
    switch (tagClass) {
      case 'UNIVERSAL':
        return '#1976d2' // Blue
      case 'APPLICATION':
        return '#d32f2f' // Red
      case 'CONTEXT':
        return '#388e3c' // Green
      case 'PRIVATE':
        return '#f57c00' // Orange
      case 'PEM':
        return '#7b1fa2' // Purple
      default:
        return '#616161' // Gray
    }
  }

  return (
    <Box sx={{ my: 0.2, fontFamily: '"Courier New", monospace' }}>
      <Box 
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        sx={{ 
          display: 'flex',
          alignItems: 'center',
          py: 0.25,
          px: 1,
          borderRadius: 1,
          cursor: 'pointer',
          transition: 'all 0.2s',
          '&:hover': {
            bgcolor: 'rgba(102, 126, 234, 0.12)',
            transform: 'translateX(2px)'
          }
        }}
      >
        {hasChildren ? (
          <IconButton 
            size="small" 
            onClick={toggleCollapse}
            sx={{ p: 0, mr: 0.5, width: 16, height: 16 }}
          >
            {isCollapsed ? <ChevronRight sx={{ fontSize: 14 }} /> : <ExpandMore sx={{ fontSize: 14 }} />}
          </IconButton>
        ) : (
          <Box sx={{ width: 24 }} />
        )}

        <Typography 
          component="span" 
          sx={{ 
            fontWeight: 600, 
            mr: 1,
            fontSize: '0.65rem',
            lineHeight: 1.2,
            color: getTagClassColor(node.tag_class)
          }}
        >
          {node.label.replace(/\s*\(Tag \d+\)/, '')}
        </Typography>

        <Typography 
          component="span" 
          sx={{ 
            color: 'text.secondary', 
            fontSize: '0.6rem',
            fontFamily: 'inherit',
            lineHeight: 1.2
          }}
        >
          (O: {node.byte_offset?.toString(16).padStart(4, '0') || '0000'}, L: {node.length})
        </Typography>

        {node.value && (
          <Typography 
            component="span" 
            sx={{ 
              color: '#0066cc', 
              fontSize: '0.6rem',
              ml: 1,
              fontFamily: 'inherit',
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
              lineHeight: 1.2
            }}
          >
            = {node.value}
          </Typography>
        )}
      </Box>

      {hasChildren && !isCollapsed && (
        <Box sx={{ 
          ml: 2.5,
          pl: 1,
          borderLeft: '1px solid #ddd'
        }}>
          {node.children.map((child, index) => (
            <TreeNode key={index} node={child} onNodeHover={onNodeHover} />
          ))}
        </Box>
      )}
    </Box>
  )
}

export default TreeNode
