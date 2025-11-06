export default function Tile({ letter, state }: { letter: string, state: "correct" | "present" | "absent" }) {
  return (
    <div class={`LetterTile-${state}`}>
      letter
    </div>
  );
}
