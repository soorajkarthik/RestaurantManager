create or replace function array_avg(arr integer[]) returns real as $$
declare
	tot real := 0;
	i integer;
begin
	foreach i in array arr
	loop
		tot = tot + i;
	end loop;
	return (tot/array_length(arr, 1));
end;
$$ immutable language plpgsql;

create table if not exists restaurants (
    name text primary key not null,
    ratings integer[] not null,
    avgRating real generated always as (array_avg(ratings)) stored
);