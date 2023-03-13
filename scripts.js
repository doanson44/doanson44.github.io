$(document).ready(function () {
    var symbols = [];
    $("#txtInput").on("input", function (event, isFromOther) {
        var inputTxt = $("#txtInput").val();
        var inputVal = parseFloat(inputTxt);

        if (inputVal) {
            $("#txtOutputPlus3").val(inputVal * 1.03);
            $("#txtOutputPlus5").val(inputVal * 1.05);

            $("#txtOutputMinus3").val(inputVal / 1.03);
            $("#txtOutputMinus5").val(inputVal / 1.05);
        }
        else {
            $("#txtOutputPlus3").val("");
            $("#txtOutputPlus5").val("");

            $("#txtOutputMinus3").val("");
            $("#txtOutputMinus5").val("");
        }

        if (!isFromOther) {
            $("#selectedSymbolPercent").text("");
            $("#txtRecommededLongItem").text("");
            $("#txtRecommededShortItem").text("");

            if (inputVal) {
                $("#txtStopLossForLongPercent").text("-10");
                $("#txtStopLossForShortPercent").text("-10");
                $("#txtStopLossForLong").val(inputVal / 1.01);
                $("#txtStopLossForShort").val(inputVal * 1.01);
            }
            else {
                $("#txtStopLossForLongPercent").text("");
                $("#txtStopLossForShortPercent").text("");
                $("#txtStopLossForLong").val("");
                $("#txtStopLossForShort").val("");
            }
            $("#txtSelectedSymbol").val("");
        }
    });

    function setUpLongShortValue(data) {
        $("#txtStopLossForLong").val(data.lowPrice);
        $("#txtStopLossForShort").val(data.highPrice);
        var stopLossForLong = ((data.lowPrice - data.lastPrice) * 1000 / data.lastPrice).toFixed(2);
        if (stopLossForLong >= -30 && data.priceChangePercent < 10) {
            $("#txtRecommededLongItem").text("Recommended");
        }
        else {
            $("#txtRecommededLongItem").text("");
        }
        $("#txtStopLossForLongPercent").text(stopLossForLong);
        var stopLossForShort = ((data.lastPrice - data.highPrice) * 1000 / data.lastPrice).toFixed(2);
        if (stopLossForShort >= -30 && data.priceChangePercent > 10) {
            $("#txtRecommededShortItem").text("Recommended");
        }
        else {
            $("#txtRecommededShortItem").text("");
        }
        $("#txtStopLossForShortPercent").text(stopLossForShort);
        $("#selectedSymbolPercent").text(data.priceChangePercent);
    }

    $("#txtSelectedSymbol").on("input", function () {
        var selectedOptionValue = $(this).val();

        if (symbols.length > 0) {
            var data = symbols.find(x => x.symbol === selectedOptionValue);
            if (data != null) {
                $("#txtInput").val(data.lastPrice).trigger('input', true);
                setUpLongShortValue(data);
            }
        }
    });

    $("#btnClear").on("click", function () {
        $("#txtInput").val("").trigger('input', false);
    });

    var settings = {
        "url": "https://fapi.binance.com/fapi/v1/ticker/24hr",
        "method": "GET",
        "timeout": 0,
        "headers": {
            "Content-Type": "application/json"
        },
    };

    $.ajax(settings).done(function (response) {
        $("#refreshTime").text(new Date().toLocaleString());
        var dataFilter = response.filter(x => x.symbol.toLowerCase().endsWith("usdt") || x.symbol.toLowerCase().endsWith("busd"));
        symbols = dataFilter.sort((a, b) => b.priceChangePercent - a.priceChangePercent);
        $("#datalistOptions").empty();
        dataFilter.forEach(element => {
            $("#datalistOptions").append(`<option value=\"${element.symbol}\">`);
        });

        setTimeout(() => {
            $("#dataTable").DataTable({
                data: dataFilter,
                columns: [
                    { data: 'symbol' },
                    { data: 'priceChangePercent' },
                    { data: 'lastPrice' },
                    { data: 'highPrice' },
                    { data: 'volume' }
                ],
                order: [[1, 'desc']],
                "createdRow": function (row, data, index) {
                    var stopLossForLong = ((data.lowPrice - data.lastPrice) * 1000 / data.lastPrice).toFixed(2);
                    if (stopLossForLong >= -30 && data.priceChangePercent < 10) {
                        $(row).addClass("table-success");
                    }
                    var stopLossForShort = ((data.lastPrice - data.highPrice) * 1000 / data.lastPrice).toFixed(2);
                    if (stopLossForShort >= -30 && data.priceChangePercent > 10) {
                        $(row).addClass("table-danger");
                    }
                },
                select: true
            });

            $("#dataTable tbody").on('click', 'tr', function () {
                var table = $("#dataTable").DataTable();
                var data = table.row(this).data();
                $("#txtInput").val(data.lastPrice).trigger('input', true);
                $("#txtSelectedSymbol").val(data.symbol);
                setUpLongShortValue(data);
            });
        }, 2000);
    });

    $("#btnRefresh").on('click', function () {
        $.ajax(settings).done(function (response) {
            $("#refreshTime").text(new Date().toLocaleString());
            var dataFilter = response.filter(x => x.symbol.toLowerCase().endsWith("usdt") || x.symbol.toLowerCase().endsWith("busd"));
            symbols = dataFilter.sort((a, b) => b.priceChangePercent - a.priceChangePercent);
            var table = $("#dataTable").DataTable();
            table.clear().draw();
            table.rows.add(dataFilter);
            table.columns.adjust().draw();

            $("#datalistOptions").empty();
            dataFilter.forEach(element => {
                $("#datalistOptions").append(`<option value=\"${element.symbol}\">`);
            });
        });
    });
});