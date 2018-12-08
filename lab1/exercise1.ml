open Core

exception Unimplemented

let main () =
  let rec gcd (m : int) (n : int) : int =
    if n = 0 then m
    else
      gcd n (m mod n)
  in

  assert (gcd 5 2 = 1);
  assert (gcd 10 2 = 2);
  assert (gcd 48 18 = 6);

  let rec fizz_buzz (n : int) : unit =
    for i = 0 to n do
        if i mod 3 = 0 && i mod 5 = 0
          then Printf.printf "fizzbuzz\n"
        else if i mod 3 = 0
          then Printf.printf "fizz\n"
        else if i mod 5 = 0
          then Printf.printf "buzz\n"
    done
  in

  let read_line () : string =
    match In_channel.input_line In_channel.stdin with
    | Some s -> s
    | None -> assert false
  in

  let rec read_password (password : string) : unit =
    if read_line () = password
      then ()
    else
      read_password password
  in

  let substring_match (pattern : string) (source : string) : int option =
    let n = String.length pattern in
      let rec recur i =
        if n + i > String.length source
          then None
        else if String.slice source i (n + i) = pattern
          then Some i
        else
          recur (i + 1)
      in
      recur 0
  in

  assert (substring_match "foo" "foobar" = Some 0);
  assert (substring_match "foo" "barfoo" = Some 3);
  assert (substring_match "z" "foobar" = None);

  ()

let () = main ()
