import { Button } from './Button'

export function DebugButton() {
  return (
    <Button
      onClick={() => {
        throw new Error('Throw Exception Test 💥')
      }}
    >
      Throw from @gitmono/ui
    </Button>
  )
}
