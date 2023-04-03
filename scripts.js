$(document).ready(function () {
    var symbols = [];
    var followingSymbols = {};

    function getFollowingSymbols() {
        var settingGetFollowingSymbols = {
            "url": "https://api.jsonbin.io/v3/b/64270f93ebd26539d0a1b9af/latest",
            "method": "GET",
            "timeout": 0,
            "headers": {
                "X-ACCESS-KEY": "$2b$10$NR1UvucZPPEZiijI5Cnv0eXaiaT2dkk9fxfev2uLRI10nJ/Z.20IG"
            },
        };

        $.ajax(settingGetFollowingSymbols).done(function (responseFollowings) {
            setTimeout(() => {
                if (!responseFollowings) {
                    return;
                }

                followingSymbols = responseFollowings.record.following;
                
                var settingsTickerPrices = {
                    "url": "https://fapi.binance.com/fapi/v1/ticker/price",
                    "method": "GET",
                    "timeout": 0,
                    "headers": {
                      "Content-Type": "application/json"
                    },
                  };
                  
                  $.ajax(settingsTickerPrices).done(function (response) {
                    var followingSymbolsTable = [];
                    for(var item in followingSymbols) {
                        var dataSymbol = response.filter(x => x.symbol === item.symbol);
                        var current = dataSymbol.price;
                        var profitLossPercent = 
                    }
                  });
            }, 1000);
        });
    }

    function updateFollowingSymbols() {
        var settingUpdateFollowingSymbol = {
            "url": "https://api.jsonbin.io/v3/b/64270f93ebd26539d0a1b9af",
            "method": "PUT",
            "timeout": 0,
            "headers": {
                "X-ACCESS-KEY": "$2b$10$NR1UvucZPPEZiijI5Cnv0eXaiaT2dkk9fxfev2uLRI10nJ/Z.20IG",
                "Content-Type": "application/json"
            },
            "data": JSON.stringify(followingSymbols),
        };

        $.ajax(settingUpdateFollowingSymbol).done(function (response) {
            console.log(response);
        });
    }

    function updateFollowingTable(data) {

    }

    getFollowingSymbols();
    $("#btnFollow").prop('disabled', true);
    $("#txtInput").on("input", function (event, isFromOther) {
        var inputTxt = $("#txtInput").val();
        var inputVal = parseFloat(inputTxt);

        if (inputVal) {
            $("#txtOutputPlus3").val(inputVal * 1.03);
            $("#txtOutputPlus5").val(inputVal * 1.05);
            $("#txtOutputPlus10").val(inputVal * 1.1);

            $("#txtOutputMinus3").val(inputVal / 1.03);
            $("#txtOutputMinus5").val(inputVal / 1.05);
            $("#txtOutputMinus10").val(inputVal / 1.1);
        }
        else {
            $("#txtOutputPlus3").val("");
            $("#txtOutputPlus5").val("");
            $("#txtOutputPlus10").val("");

            $("#txtOutputMinus3").val("");
            $("#txtOutputMinus5").val("");
            $("#txtOutputMinus10").val("");
        }

        if (!isFromOther) {
            $("#selectedSymbolPercent").text("");
            $("#selectedSymbolHighest").text("");
            $("#selectedSymbolLowest").text("");
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

        var selectedSymbol = $("#txtSelectedSymbol").val();
        if (selectedSymbol) {
            $("#btnFollow").prop('disabled', false);
            $("#txtSelectedSymbolToAdd").val(selectedSymbol);
            $("#txtEntry").val(inputVal);
            $("#txtAmount").val(50);
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

        var settingsKlines = {
            "url": `https://fapi.binance.com/fapi/v1/klines?symbol=${data.symbol}&interval=1d&limit=100`,
            "method": "GET",
            "timeout": 0,
            "headers": {
                "Content-Type": "application/json"
            },
        };

        $.ajax(settingsKlines).done(function (response) {
            setTimeout(() => {
                var highest = Math.max(...response.map(x => x[2]));
                var lowest = Math.min(...response.map(x => x[3]))
                $("#selectedSymbolHighest").text(highest);
                $("#selectedSymbolLowest").text(lowest);
            }, 1000);
        });
    }

    $("#txtSelectedSymbol").on("input", function () {
        var selectedOptionValue = $(this).val();

        if (symbols.length > 0) {
            var data = symbols.find(x => x.symbol === selectedOptionValue);
            if (data != null) {
                $("#txtInput").val(data.lastPrice).trigger('input', true);
                setUpLongShortValue(data);
            }
            else {
                $("#btnFollow").prop('disabled', true);
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