module rust_module
    use iso_c_binding
    implicit none

    interface
        function rust_add(a, b) result(result__) bind(C, name="rust_add")
            use iso_c_binding
            integer(c_int), value, intent(in) :: a
            integer(c_int), value, intent(in) :: b
            integer(c_int32_t) :: result__
        end function rust_add

    end interface
end module rust_module
