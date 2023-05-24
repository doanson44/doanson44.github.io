$(document).ready(function () {
    var symbols = [];
    var cryptocurrencies = {};
    var stocks = {};
    var longSymbols = [];
    var shortSymbols = [];

    function Record(cryptocurrencies, stocks) {
        this.cryptocurrencies = cryptocurrencies;
        this.stocks = stocks;
    }

    function Symbol(symbol, entry, amount, type) {
        this.symbol = symbol;
        this.entry = entry;
        this.amount = amount;
        this.type = type;
        this.dateTime = new Date();
    }

    function FollowingSymbol(name, type, entry, current, amount, profitLossPercent, profitLossValue, dateTime) {
        this.name = name;
        this.type = type;
        this.entry = entry;
        this.current = current;
        this.amount = amount;
        this.profitLossPercent = profitLossPercent;
        this.profitLossValue = profitLossValue;
        this.dateTime = dateTime;
    }

    function updateCryptocurrenciesTable(init = true) {
        $("#refreshTimeFollowing").text(new Date().toLocaleString());

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
            for (var index in cryptocurrencies) {
                var item = cryptocurrencies[index];
                var dataSymbol = response.find(x => x.symbol === item.symbol);

                if (dataSymbol) {
                    var current = dataSymbol.price;
                    var profitLossPercent = 0;
                    var profitLossValue = 0;
                    if (item.type.toLowerCase() === "long") {
                        profitLossPercent = (current - item.entry) / item.entry * 1000;
                        profitLossValue = profitLossPercent * item.amount / 100;
                    } else {
                        profitLossPercent = (item.entry - current) / item.entry * 1000;
                        profitLossValue = profitLossPercent * item.amount / 100;
                    }
                    followingSymbolsTable.push(new FollowingSymbol(item.symbol, item.type, item.entry, current, item.amount, profitLossPercent.toFixed(2), profitLossValue.toFixed(2), item.dateTime));
                }
            }

            if (followingSymbolsTable) {
                var table = null;
                if (init) {
                    table = $("#dataTableFollowing").DataTable({
                        data: followingSymbolsTable,
                        columns: [
                            { data: 'name' },
                            { data: 'type' },
                            { data: 'entry' },
                            { data: 'current' },
                            { data: 'amount' },
                            { data: 'profitLossPercent' },
                            { data: 'profitLossValue' },
                            { data: 'dateTime' }
                        ],
                        columnDefs: [
                            {
                                targets: 8,
                                data: null,
                                defaultContent: '<button class="btn btn-sm btn-danger">Remove</button>',
                            },
                        ],
                        "createdRow": function (row, data, index) {
                            if (data.profitLossValue > 0) {
                                $(row).addClass("table-success");
                            }
                            else {
                                $(row).addClass("table-danger");
                            }
                        },
                        order: [[0, 'asc']]
                    });
                } else {
                    table = $("#dataTableFollowing").DataTable();
                    table.clear().draw();
                    table.rows.add(followingSymbolsTable);
                    table.columns.adjust().draw();
                }

                $('#dataTableFollowing tbody').on('click', 'button', function () {
                    var data = table.row(this).data();
                    if (data) {
                        cryptocurrencies = cryptocurrencies.filter(x => x.symbol !== data.name);
                        updateFollowingSymbols();
                    }
                });

                const total = followingSymbolsTable.reduce((accumulator, object) => {
                    return accumulator + parseFloat(object.profitLossValue);
                }, 0);

                if (total > 0) {
                    $("#txtProfitLossTotalClass").addClass("alert-success");
                }
                else {
                    $("#txtProfitLossTotalClass").addClass("alert-danger");
                }

                $("#txtProfitLossTotal").text(total.toFixed(2));
            }
        });
    }

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
            if (!responseFollowings) {
                return;
            }

            cryptocurrencies = responseFollowings.record.cryptocurrencies;
            stocks = responseFollowings.record.stocks;
            updateCryptocurrenciesTable();
        });
    }

    function updateFollowingSymbols() {
        var data = new Record(cryptocurrencies, stocks);
        var settingUpdateFollowingSymbol = {
            "url": "https://api.jsonbin.io/v3/b/64270f93ebd26539d0a1b9af",
            "method": "PUT",
            "timeout": 0,
            "headers": {
                "X-ACCESS-KEY": "$2b$10$NR1UvucZPPEZiijI5Cnv0eXaiaT2dkk9fxfev2uLRI10nJ/Z.20IG",
                "Content-Type": "application/json"
            },
            "data": JSON.stringify(data),
        };

        $.ajax(settingUpdateFollowingSymbol).done(function (response) {
            cryptocurrencies = response.record.cryptocurrencies;
            stocks = response.record.stocks;
            setTimeout(updateCryptocurrenciesTable(false), 500);
        });
    }

    $("#btnSaveChanges").on("click", function () {
        if (!cryptocurrencies) {
            cryptocurrencies = {};
        }
        var symbol = $("#txtSelectedSymbolToAdd").val();
        var entry = parseFloat($("#txtEntry").val());
        var amount = parseFloat($("#txtAmount").val());
        var type = $("#selectType :selected").val();
        var item = new Symbol(symbol, entry, amount, type);
        cryptocurrencies.push(item);
        updateFollowingSymbols();
        $('#followModal').modal('hide');
    });

    $("#btnRefreshFollowing").on("click", function () {
        setTimeout(() => {
            updateCryptocurrenciesTable(false);
        }, 500);
    });

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
            $("#txtAmount").val(5);
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
                    if (stopLossForLong >= -30 && data.priceChangePercent <= -10) {
                        $(row).addClass("table-success");
                        if (!longSymbols.includes(data.symbol)) {
                            notificationNow(`Good entry to long: ${data.symbol} at ${data.lastPrice}`);
                            longSymbols.push(data.symbol);
                        }
                    }
                    var stopLossForShort = ((data.lastPrice - data.highPrice) * 1000 / data.lastPrice).toFixed(2);
                    if (stopLossForShort >= -50 && data.priceChangePercent >= 20) {
                        $(row).addClass("table-danger");
                        if (!shortSymbols.includes(data.symbol)) {
                            notificationNow(`Good entry to short: ${data.symbol} at ${data.lastPrice}`);
                            shortSymbols.push(data.symbol);
                        }
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
        }, 500);
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

    function notificationNow(message) {
        var isEnableNotification = $('#btnEnableNotification').is(":checked");

        if (!window.Notification) {
            console.log('Browser does not support notifications.');
        } else if (isEnableNotification) {
            // check if permission is already granted
            if (Notification.permission === 'granted') {
                // show notification here
                var notify = new Notification('Hi there!', {
                    body: message
                });
            } else {
                // request permission from user
                Notification.requestPermission().then(function (p) {
                    if (p === 'granted') {
                        // show notification here
                        var notify = new Notification('Hi there!', {
                            body: message
                        });
                    } else {
                        console.log('User blocked notifications.');
                    }
                }).catch(function (err) {
                    console.error(err);
                });
            }
        }
    }

    function InitView() {
        getFollowingSymbols();
    }

    InitView();
    window.setInterval(() => {
        $("#btnRefresh").trigger("click");
        $("#btnRefreshFollowing").trigger("click");
    }, 300000);
});