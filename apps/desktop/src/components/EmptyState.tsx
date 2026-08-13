export function EmptyState({
  title,
  message,
}: {
  title: string;
  message: string;
}) {
  return (
    <div className="empty-state">
      <h2>{title}</h2>
      <p>{message}</p>
    </div>
  );
}
