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

  return (
    <Box sx={{ my: 0.5, fontFamily: '"Courier New", monospace' }}>
      <Box 
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        sx={{ 
          display: 'flex',
          alignItems: 'center',
          py: 0.5,
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
            sx={{ p: 0, mr: 0.5, width: 20, height: 20 }}
          >
            {isCollapsed ? <ChevronRight sx={{ fontSize: 16 }} /> : <ExpandMore sx={{ fontSize: 16 }} />}
          </IconButton>
        ) : (
          <Box sx={{ width: 28 }} />
        )}

        <Typography 
          component="span" 
          sx={{ 
            fontWeight: 600, 
            mr: 1,
            fontSize: '0.75rem',
            lineHeight: 1.4
          }}
        >
          {node.label.replace(/\s*\(Tag \d+\)/, '')}
        </Typography>

        <Typography 
          component="span" 
          sx={{ 
            color: 'text.secondary', 
            fontSize: '0.7rem',
            mr: 1,
            fontFamily: 'inherit'
          }}
        >
          @{node.byte_offset?.toString(16).padStart(4, '0') || '0000'}
        </Typography>

        <Typography 
          component="span" 
          sx={{ 
            color: 'text.secondary', 
            fontSize: '0.7rem',
            fontFamily: 'inherit'
          }}
        >
          [{node.length}B]
        </Typography>

        {node.value && (
          <Typography 
            component="span" 
            sx={{ 
              color: '#0066cc', 
              fontSize: '0.7rem',
              ml: 1,
              fontFamily: 'inherit',
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap'
            }}
          >
            = {node.value}
          </Typography>
        )}
      </Box>

      {hasChildren && !isCollapsed && (
        <Box sx={{ 
          ml: 3,
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
