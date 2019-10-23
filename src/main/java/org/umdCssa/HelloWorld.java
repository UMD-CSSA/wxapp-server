package org.umdCssa;

import com.firenio.codec.http11.HttpCodec;
import com.firenio.codec.http11.HttpFrame;
import com.firenio.codec.http11.HttpHeader;
import com.firenio.codec.http2.Http2Codec;
import com.firenio.codec.lengthvalue.LengthValueCodec;
import com.firenio.component.Channel;
import com.firenio.component.ChannelAcceptor;
import com.firenio.component.Frame;
import com.firenio.component.IoEventHandle;
import com.firenio.component.LoggerChannelOpenListener;
import com.firenio.component.NioEventLoopGroup;

import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.util.Date;
import java.util.Optional;

public class HelloWorld {

    public static void main(@NotNull final String[] args) throws Exception {

        final var ioEventHandle = new IoEventHandle() {

            @Override
            public void accept(@NotNull final Channel channel,
                               @NotNull final Frame frame) throws Exception {
                final var text = Optional.ofNullable(
                        frame.getStringContent())
                        .orElse("[INFO] Nothing received.");
                frame.setContent(channel.allocate());

                final String content = "Hello World!\n" +
                        text + "\n\n" +
                        frame.getFrameName() + '\n' +
                        ZonedDateTime.now().format(
                                DateTimeFormatter.ISO_ZONED_DATE_TIME);
                frame.write(content, channel);
                channel.writeAndFlush(frame);
            }
        };

        final NioEventLoopGroup group = new NioEventLoopGroup();
        group.setEnableMemoryPoolDirect(true);
        final var context = new ChannelAcceptor(group, 6006);
        context.addChannelEventListener(new LoggerChannelOpenListener());
        context.setIoEventHandle(ioEventHandle);
        context.addProtocolCodec(new HttpCodec());
        context.bind();
    }
}
