export const Loading = () => {
  return (
    <div className="flex justify-center items-center min-h-[200px]">
      <div className="animate-spin rounded-full h-10 w-10 border-t-2 border-b-2 border-gray-600"></div>
      <span className="ml-4 text-gray-700 font-medium">Loading...</span>
    </div>
  );
};
