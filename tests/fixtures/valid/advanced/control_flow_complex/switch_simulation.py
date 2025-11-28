# Switch-case simulation with if-elif chains
def day_of_week(n: int) -> int:
    result: int = 0

    if n == 1:
        result = 100
    else:
        if n == 2:
            result = 200
        else:
            if n == 3:
                result = 300
            else:
                if n == 4:
                    result = 400
                else:
                    if n == 5:
                        result = 500
                    else:
                        if n == 6:
                            result = 600
                        else:
                            if n == 7:
                                result = 700
                            else:
                                result = 0

    return result

def month_days(month: int) -> int:
    days: int = 0

    if month == 1:
        days = 31
    else:
        if month == 2:
            days = 28
        else:
            if month == 3:
                days = 31
            else:
                if month == 4:
                    days = 30
                else:
                    if month == 5:
                        days = 31
                    else:
                        if month == 6:
                            days = 30
                        else:
                            days = 31

    return days

sum_days: int = 0
i: int = 1
while i <= 7:
    sum_days = sum_days + day_of_week(i)
    i = i + 1

sum_months: int = 0
j: int = 1
while j <= 6:
    sum_months = sum_months + month_days(j)
    j = j + 1

print("Sum of day codes:", sum_days)
print("Sum of month days:", sum_months)
