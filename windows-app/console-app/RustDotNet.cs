using System;
using System.Runtime.InteropServices;

namespace ConsoleApplication1
{
    class RustDotNet
    {
        public static void Main(string[] args)
        {
            Console.WriteLine("Running Rust code in a .NET application");
            Console.WriteLine("Calling rust! {0}", HelloRust.foo(3, 5));

            HelloRust.say_hello("World!");
            Console.Read();
        }
    }

    static class HelloRust
    {
        [DllImport("hellorust.dll")]
        public static extern uint foo(uint value1, uint value2);

        [DllImport("hellorust.dll")]
        public static extern void say_hello(string name);
    }
}
